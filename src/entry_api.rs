use std::borrow::Borrow;
use std::fmt;
use std::fmt::Debug;
#[cfg(feature = "raw_entry")]
use std::marker::PhantomData;
use Entry::*;
use crate::{CollectionMut, CollectionRef, KeyedRef};

/// The type passed into [`EntryTypes`] to provied the [`Entry`] for [`EntryApi`]
pub struct EntryFlag;
/// The type passed into [`EntryTypes`] to provied the [`Entry`] for [`EntryRefApi<Q>`]
/// This type should never implement [`SupportsKeyVacantEntry`] to avoid logic errors
pub struct EntryRefFlag<Q: ?Sized>(*const Q);

/// A marker trait marking whether [`EntryTypes<T>::Vacant`] supports key operations
/// eg. since [`EntryFlag`] implements this marker the entry returned by [`EntryApi`] supports key operations
pub trait SupportsKeyVacantEntry {}
impl SupportsKeyVacantEntry for EntryFlag {}

/// Trait for types which can have [`Entry`]s associated with them.
/// The `T` parameter allows one type to have multiple associated [`Entry`]s
/// see also [`EntryFlag`], [`EntryRefFlag`]
pub trait EntryTypes<T>: CollectionMut + CollectionRef + KeyedRef {
	type Occupied<'a>: OccupiedEntry<'a, Self>
	where
	Self: 'a, T: 'a;
	type Vacant<'a>: VacantEntry<'a, Self, T>
	where
	Self: 'a, T: 'a;
}

/// Mutable map that supports the entry api
pub trait EntryApi: EntryTypes<EntryFlag> {
	/// Gets the given key's corresponding entry in the map for in-place manipulation.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::{entry_api::*, Get};
	///
	/// fn make_map() -> impl EntryApi<Key=char,Item=u32> + Index<&'static char, Output=u32> + Get<&'static char> { HashMap::new() }
	/// let mut letters = make_map();
	///
	/// for ch in "a short treatise on fungi".chars() {
	///     let mut counter = letters.entry(ch).or_insert(0);
	///     *counter += 1;
	/// }
	///
	/// assert_eq!(letters[&'s'], 2);
	/// assert_eq!(letters[&'t'], 3);
	/// assert_eq!(letters[&'u'], 1);
	/// assert!(!letters.contains_key(&'y'));
	/// ```
	fn entry(&mut self, key: Self::Key) -> Entry<'_, Self, EntryFlag>;
}


pub trait EntryRefApi<Q: ?Sized>: EntryTypes<EntryRefFlag<Q>>
	where Self::Key: Borrow<Q> {
	/// Gets the given key's corresponding entry in the map for in-place manipulation.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::{entry_api::*, Get};
	///
	/// fn make_map() -> impl EntryRefApi<str, Key=String,Item=u32> + Index<&'static str, Output=u32> + Get<&'static str> { HashMap::new() }
	/// let mut words = make_map();
	///
	/// for s in "foo bar bar".split(' ') {
	///     let mut counter = words.entry_ref(s).or_insert(0);
	///     *counter += 1;
	/// }
	///
	/// assert_eq!(words["bar"], 2);
	/// assert_eq!(words["foo"], 1);
	/// assert!(!words.contains_key("baz"));
	/// ```
	///
	/// ```compile_fail
	/// use std::collections::HashMap;
	/// use cc_traits::{EntryRefApi, entry_api::*};
	///
	/// fn make_map() -> impl EntryRefApi<String, Key=String, Item=String>{ HashMap::new() }
	///
	/// let mut test = make_map();
	/// let s = "hello world".to_string();
	/// test.entry_ref(&s).or_insert(s); // the reference to s is still required so it can't be moved to insert
	/// ```
	/// Note: implementing this trait for hash map requires the `raw_entry` feature since it makes use of the `hash_raw_entry` nightly feature
	fn entry_ref<'a>(&'a mut self, key: &'a Q) -> Entry<'a, Self, EntryRefFlag<Q>>
		where Q: 'a;
}

/// A view into an occupied entry.
/// It is part of the [`Entry`] enum.
pub trait OccupiedEntry<'a, C: CollectionMut + CollectionRef + KeyedRef + ?Sized>: Sized {
	/// Gets a reference to the key in the entry.
	///
	/// # Examples
	///
	/// ```
	///use std::collections::HashMap;
	///use std::cmp::PartialEq;
	///use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	/// assert!(map.entry("poneyland").key().eq("poneyland"));
	/// ```
	fn key(&self) -> C::KeyRef<'_>;

	/// Take the ownership of the key and value from the map.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::{entry_api::*, Get};
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> + Get<&'static str> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	///
	/// if let Entry::Occupied(o) = map.entry("poneyland") {
	///     // We delete the entry from the map.
	///     o.remove_entry();
	/// }
	///
	/// assert_eq!(map.contains_key("poneyland"), false);
	/// ```
	fn remove_entry(self) -> (C::Key, C::Item);

	/// Gets a reference to the value in the entry.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::cmp::Eq;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	///
	/// if let Entry::Occupied(o) = map.entry("poneyland") {
	///     assert!(o.get().eq(&12));
	/// };
	/// ```
	fn get(&self) -> C::ItemRef<'_>;

	/// Gets a mutable reference to the value in the entry.
	///
	/// If you need a reference to the `OccupiedEntry` which may outlive the
	/// destruction of the `Entry` value, see [`into_mut`].
	///
	/// [`into_mut`]: Self::into_mut
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> + Index<&'static str, Output=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	///
	/// assert_eq!(map["poneyland"], 12);
	/// if let Entry::Occupied(mut o) = map.entry("poneyland") {
	///     *o.get_mut() += 10;
	///     assert_eq!(*o.get(), 22);
	///
	///     // We can use the same Entry multiple times.
	///     *o.get_mut() += 2;
	/// }
	///
	/// assert_eq!(map["poneyland"], 24);
	/// ```
	fn get_mut(&mut self) -> C::ItemMut<'_>;

	/// Converts the `OccupiedEntry` into a mutable reference to the value in the entry
	/// with a lifetime bound to the map itself.
	///
	/// If you need multiple references to the `OccupiedEntry`, see [`get_mut`].
	///
	/// [`get_mut`]: Self::get_mut
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> + Index<&'static str, Output=i32>{ HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	///
	/// assert_eq!(map["poneyland"], 12);
	/// if let Entry::Occupied(o) = map.entry("poneyland") {
	///     *o.into_mut() += 10;
	/// }
	///
	/// assert_eq!(map["poneyland"], 22);
	/// ```
	fn into_mut(self) -> C::ItemMut<'a>;

	/// Sets the value of the entry, and returns the entry's old value.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> + Index<&'static str, Output=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	///
	/// if let Entry::Occupied(mut o) = map.entry("poneyland") {
	///     assert_eq!(o.insert(15), 12);
	/// }
	///
	/// assert_eq!(map["poneyland"], 15);
	/// ```
	fn insert(&mut self, value: C::Item) -> C::Item;

	/// Takes the value out of the entry, and returns it.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::{entry_api::*, Get};
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> + Get<&'static str> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	///
	/// if let Entry::Occupied(o) = map.entry("poneyland") {
	///     assert_eq!(o.remove(), 12);
	/// }
	///
	/// assert_eq!(map.contains_key("poneyland"), false);
	/// ```
	fn remove(self) -> C::Item {
		self.remove_entry().1
	}
}

/// A view into a vacant entry.
/// It is part of the [`Entry`] enum.
/// `T` is just used to determine whether to support key operations
pub trait VacantEntry<'a, C: CollectionMut + CollectionRef + KeyedRef + ?Sized, T=EntryFlag>: Sized {
	/// Sets the value of the entry with the `VacantEntry`'s key,
	/// and returns a mutable reference to it.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> + Index<&'static str, Output=i32> { HashMap::new() }
	/// let mut map = make_map();
	///
	/// if let Entry::Vacant(o) = map.entry("poneyland") {
	///     o.insert(37);
	/// }
	/// assert_eq!(map["poneyland"], 37);
	/// ```
	fn insert(self, value: C::Item) -> C::ItemMut<'a>;

	/// Gets a reference to the key that would be used when inserting a value
	/// through the `VacantEntry`.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::cmp::PartialEq;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// assert!(map.entry("poneyland").key().eq("poneyland"));
	/// ```
	fn key(&self) -> C::KeyRef<'_>
	where T: SupportsKeyVacantEntry;

	/// Take ownership of the key.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	///
	/// if let Entry::Vacant(v) = map.entry("poneyland") {
	///     v.into_key();
	/// };
	/// ```
	fn into_key(self) -> C::Key
		where T: SupportsKeyVacantEntry;

}



///A view into a single entry in a map, which may either be vacant or occupied.
///
/// This enum is constructed from the entry method on [`EntryApi`][`crate::EntryApi`] implementors,
/// as well as the entry_ref method of [`EntryRefApi`][`crate::EntryRefApi`] implementors.
///
/// Note: while this enum is distinct from Entry enums defined for different map types,
/// (eg. [`std::collections::hash_map::Entry`])
/// it supports the same functionality and has the same variant elements
/// ```
/// use cc_traits::entry_api::*;
/// use std::collections::hash_map::{HashMap, VacantEntry as HashMapVacantEntry};
///
/// let mut x: HashMap<&'static str, ()> = HashMap::new();
/// if let Entry::Vacant(y) =  EntryApi::entry(&mut x, "test") {
///     let z: HashMapVacantEntry<_,_> = y;
/// }
/// ```
pub enum Entry<'a, C: EntryTypes<T> + 'a + ?Sized, T:'a=EntryFlag> {
	/// An occupied entry.
	Occupied(<C as EntryTypes<T>>::Occupied<'a>),
	/// A vacant entry.
	Vacant(<C as EntryTypes<T>>::Vacant<'a>),
}

impl<'a, C: EntryTypes<T> + 'a + ?Sized, T> Entry<'a, C, T>
{
	/// Ensures a value is in the entry by inserting the default if empty, and returns
	/// a mutable reference to the value in the entry.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=u32> + Index<&'static str, Output=u32> { HashMap::new() }
	/// let mut map = make_map();
	///
	/// map.entry("poneyland").or_insert(3);
	/// assert_eq!(map["poneyland"], 3);
	///
	/// *map.entry("poneyland").or_insert(10) *= 2;
	/// assert_eq!(map["poneyland"], 6);
	/// ```
	#[inline]
	pub fn or_insert(self, default: C::Item) -> C::ItemMut<'a> {
		match self {
			Occupied(entry) => entry.into_mut(),
			Vacant(entry) => entry.insert(default),
		}
	}

	/// Ensures a value is in the entry by inserting the result of the default function if empty,
	/// and returns a mutable reference to the value in the entry.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=String> + Index<&'static str, Output=String> { HashMap::new() }
	/// let mut map = make_map();
	/// let s = "hoho".to_string();
	///
	/// map.entry("poneyland").or_insert_with(|| s);
	///
	/// assert_eq!(map["poneyland"], "hoho".to_string());
	/// ```
	#[inline]
	pub fn or_insert_with<F: FnOnce() -> C::Item>(self, default: F) -> C::ItemMut<'a> {
		match self {
			Occupied(entry) => entry.into_mut(),
			Vacant(entry) => entry.insert(default()),
		}
	}

	/// Provides in-place mutable access to an occupied entry before any
	/// potential inserts into the map.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=u32> + Index<&'static str, Output=u32> { HashMap::new() }
	/// let mut map = make_map();
	///
	/// map.entry("poneyland")
	///    .and_modify(|mut e| { *e += 1 })
	///    .or_insert(42);
	/// assert_eq!(map["poneyland"], 42);
	///
	/// map.entry("poneyland")
	///    .and_modify(|mut e| { *e += 1 })
	///    .or_insert(42);
	/// assert_eq!(map["poneyland"], 43);
	/// ```
	#[inline]
	pub fn and_modify<F>(self, f: F) -> Self
	where
		F: FnOnce(C::ItemMut<'_>),
	{
		match self {
			Occupied(mut entry) => {
				f(entry.get_mut());
				Occupied(entry)
			}
			Vacant(entry) => Vacant(entry),
		}
	}
}

impl<'a, C: EntryTypes<T> + 'a + ?Sized, T: SupportsKeyVacantEntry> Entry<'a, C, T>
{
	/// Returns a reference to this entry's key.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::entry_api::*;
	/// use std::cmp::PartialEq;
	///
	/// fn make_map() -> impl for<'a> EntryApi<Key=&'static str,Item=u32> { HashMap::new() }
	/// let mut map = make_map();
	/// assert!(map.entry("poneyland").key().eq("poneyland"));
	/// ```
	#[inline]
	pub fn key(&self) -> C::KeyRef<'_> {
		match *self {
			Occupied(ref entry) => entry.key(),
			Vacant(ref entry) => entry.key(),
		}
	}

	/// Ensures a value is in the entry by inserting, if empty, the result of the default function.
	/// This method allows for generating key-derived values for insertion by providing the default
	/// function a reference to the key that was moved during the `.entry(key)` method call.
	///
	/// The reference to the moved key is provided so that cloning or copying the key is
	/// unnecessary, unlike with `.or_insert_with(|| ... )`.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=usize> + Index<&'static str, Output=usize> { HashMap::new() }
	/// let mut map = make_map();
	///
	/// map.entry("poneyland").or_insert_with_key(|key| key.chars().count());
	///
	/// assert_eq!(map["poneyland"], 9);
	/// ```
	#[inline]
	pub fn or_insert_with_key<F: FnOnce(C::KeyRef<'_>) -> C::Item>(self, default: F) -> C::ItemMut<'a> {
		match self {
			Occupied(entry) => entry.into_mut(),
			Vacant(entry) => {
				let value = default(entry.key());
				entry.insert(value)
			}
		}
	}
}

impl<'a, C: EntryTypes<T> + 'a + ?Sized, T> Entry<'a, C, T>
where
	C::Item: Default,
{
	/// Ensures a value is in the entry by inserting the default value if empty,
	/// and returns a mutable reference to the value in the entry.
	///
	/// # Examples
	///
	/// ```
	/// # fn main() {
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=Option<u32>> + Index<&'static str, Output=Option<u32>> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_default();
	///
	/// assert_eq!(map["poneyland"], None);
	/// # }
	/// ```
	#[inline]
	pub fn or_default(self) -> C::ItemMut<'a> {
		match self {
			Occupied(entry) => entry.into_mut(),
			Vacant(entry) => entry.insert(Default::default()),
		}
	}
}

impl<'a, C: EntryTypes<T> + 'a + ?Sized, T> Debug for Entry<'a, C, T>
where C::Occupied<'a>: Debug, C::Vacant<'a>: Debug
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Vacant(v) => f.debug_tuple("Entry").field(v).finish(),
			Occupied(o) => f.debug_tuple("Entry").field(o).finish(),
		}
	}
}

#[cfg(feature = "raw_entry")]
pub struct RefOccupiedEntry<Occ, C: ?Sized>(Occ, PhantomData<C>);

#[cfg(feature = "raw_entry")]
impl<Occ, C: ?Sized> RefOccupiedEntry<Occ, C> {
	#[inline(always)]
	pub fn new(x: Occ) -> Self {
		RefOccupiedEntry(x, PhantomData::default())
	}
}

#[cfg(feature = "raw_entry")]
impl<'a, Occ: OccupiedEntry<'a, C>, C: CollectionMut + CollectionRef + KeyedRef + ?Sized> OccupiedEntry<'a, C> for RefOccupiedEntry<Occ, C> {

	#[inline(always)]
	fn key(&self) -> C::KeyRef<'_> {
		self.0.key()
	}

	#[inline(always)]
	fn remove_entry(self) -> (C::Key, C::Item) {
		self.0.remove_entry()
	}

	#[inline(always)]
	fn get(&self) -> C::ItemRef<'_> {
		self.0.get()
	}

	#[inline(always)]
	fn get_mut(&mut self) -> C::ItemMut<'_> {
		self.0.get_mut()
	}

	#[inline(always)]
	fn into_mut(self) -> C::ItemMut<'a> {
		self.0.into_mut()
	}

	#[inline(always)]
	fn insert(&mut self, value: C::Item) -> C::Item {
		self.0.insert(value)
	}

	#[inline(always)]
	fn remove(self) -> C::Item {
		self.0.remove()
	}
}

#[cfg(feature = "raw_entry")]
impl<'a, Occ: OccupiedEntry<'a, C>, C: CollectionMut + CollectionRef + KeyedRef + ?Sized> Debug for RefOccupiedEntry<Occ, C>
where C::Key: Debug, C::Item: Debug {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("RefOccupiedEntry")
			.field("key", &*self.key())
			.field("value", &*self.get())
			.finish_non_exhaustive()
	}
}

#[cfg(feature = "raw_entry")]
pub trait RawVacantEntry<'a, C: CollectionMut + CollectionRef + KeyedRef + ?Sized>: Sized {
	fn insert(self, key: C::Key, value: C::Item) -> (C::KeyRef<'a>, C::ItemMut<'a>);
}

#[cfg(feature = "raw_entry")]
pub struct RefVacantEntry<Q, Vacant, C: ?Sized> {
	key: Q,
	raw: Vacant,
	_phantom: PhantomData<C>
}

#[cfg(feature = "raw_entry")]
impl<Q, Vacant, C: ?Sized> RefVacantEntry<Q, Vacant, C> {
	#[inline(always)]
	pub fn new(key: Q, raw: Vacant) -> Self {
		RefVacantEntry{key, raw, _phantom: PhantomData::default()}
	}
}

#[cfg(feature = "raw_entry")]
impl<'a, Q: ?Sized, Vac: RawVacantEntry<'a, C>, C: CollectionMut + CollectionRef + KeyedRef + ?Sized> VacantEntry<'a, C, crate::EntryRefFlag<Q>>
	for RefVacantEntry<&'a Q, Vac, C>
where Q: ToOwned<Owned=C::Key>
{

	#[inline(always)]
	fn insert(self, value: C::Item) -> C::ItemMut<'a> {
		self.raw.insert(self.key.to_owned(), value).1
	}

	fn key(&self) -> C::KeyRef<'_> where crate::EntryRefFlag<Q>: SupportsKeyVacantEntry {
		unreachable!()
		// EntryRefFlag doesn't implement SupportsKeyVacantEntry
	}

	fn into_key(self) -> C::Key where crate::EntryRefFlag<Q>: SupportsKeyVacantEntry {
		unreachable!()
	}
}

#[cfg(feature = "raw_entry")]
impl<'a, Q: Debug, Vacant: RawVacantEntry<'a, C>, C: CollectionMut + CollectionRef + KeyedRef + ?Sized> Debug
	for RefVacantEntry<&'a Q, Vacant, C>
where C::Key: Borrow<Q>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("RefVacantEntry")
			.field("key", self.key)
			.finish_non_exhaustive()
	}
}

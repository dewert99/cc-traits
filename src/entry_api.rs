use crate::{Collection, CollectionMut, CollectionRef, Keyed, KeyedRef};
use std::borrow::Borrow;
use std::fmt;
use std::fmt::Debug;
use Entry::*;

/// Mutable map that supports the entry api
pub trait EntryApi: KeyedRef + CollectionRef + CollectionMut {
	type Occupied<'a>: OccupiedEntry<
		'a,
		Key = Self::Key,
		Item = Self::Item,
		KeyRef = Self::KeyRef<'a>,
		ItemRef = Self::ItemRef<'a>,
		ItemMut = Self::ItemMut<'a>,
	> where
		Self: 'a;
	type Vacant<'a>: KeyVacantEntry<
		'a,
		Key = Self::Key,
		Item = Self::Item,
		KeyRef = Self::KeyRef<'a>,
		ItemRef = Self::ItemRef<'a>,
		ItemMut = Self::ItemMut<'a>,
	> where
		Self: 'a;

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
	fn entry(&mut self, key: Self::Key) -> Entry<Self::Occupied<'_>, Self::Vacant<'_>>;
}

pub trait EntryRefApi<Q: ?Sized>: KeyedRef + CollectionRef + CollectionMut
where
	Self::Key: Borrow<Q>,
{
	type Occupied<'a>: OccupiedEntry<
		'a,
		Key = Self::Key,
		Item = Self::Item,
		KeyRef = Self::KeyRef<'a>,
		ItemRef = Self::ItemRef<'a>,
		ItemMut = Self::ItemMut<'a>,
	> where
		Self: 'a,
		Q: 'a;
	type Vacant<'a>: VacantEntry<
		'a,
		Key = Self::Key,
		Item = Self::Item,
		KeyRef = Self::KeyRef<'a>,
		ItemRef = Self::ItemRef<'a>,
		ItemMut = Self::ItemMut<'a>,
	> where
		Self: 'a,
		Q: 'a;

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
	fn entry_ref<'a>(&'a mut self, key: &'a Q) -> Entry<Self::Occupied<'a>, Self::Vacant<'a>>;
}

pub trait AssociatedCollection {
	type Owner;
}

impl<S: AssociatedCollection> Collection for S
where
	S::Owner: Collection,
{
	type Item = <S::Owner as Collection>::Item;
}

impl<S: AssociatedCollection> Keyed for S
where
	S::Owner: Keyed,
{
	type Key = <S::Owner as Keyed>::Key;
}

impl<S: AssociatedCollection> CollectionRef for S
where
	S::Owner: CollectionRef,
{
	type ItemRef<'a>
	where
		Self: 'a,
	= <S::Owner as CollectionRef>::ItemRef<'a>;

	#[inline(always)]
	fn upcast_item_ref<'short, 'long: 'short>(r: Self::ItemRef<'long>) -> Self::ItemRef<'short>
	where
		Self: 'long,
	{
		S::Owner::upcast_item_ref(r)
	}
}

impl<S: AssociatedCollection> CollectionMut for S
where
	S::Owner: CollectionMut,
{
	type ItemMut<'a>
	where
		Self: 'a,
	= <S::Owner as CollectionMut>::ItemMut<'a>;

	fn upcast_item_mut<'short, 'long: 'short>(r: Self::ItemMut<'long>) -> Self::ItemMut<'short>
	where
		Self: 'long,
	{
		S::Owner::upcast_item_mut(r)
	}
}

impl<S: AssociatedCollection> KeyedRef for S
where
	S::Owner: KeyedRef,
{
	type KeyRef<'a>
	where
		Self: 'a,
	= <S::Owner as KeyedRef>::KeyRef<'a>;

	#[inline(always)]
	fn upcast_key_ref<'short, 'long: 'short>(r: Self::KeyRef<'long>) -> Self::KeyRef<'short>
	where
		Self: 'long,
	{
		S::Owner::upcast_key_ref(r)
	}
}

/// A view into an occupied entry.
/// It is part of the [`Entry`] enum.
pub trait OccupiedEntry<'a>: CollectionRef + CollectionMut + KeyedRef + Sized {
	/// Gets a reference to the key in the entry.
	///
	/// # Examples
	///
	/// ```
	///use std::collections::HashMap;
	///use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	/// assert_eq!(map.entry("poneyland").key(), &"poneyland");
	/// ```
	fn key(&self) -> &Self::Key;

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
	fn remove_entry(self) -> (Self::Key, Self::Item);

	/// Gets a reference to the value in the entry.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	///
	/// if let Entry::Occupied(o) = map.entry("poneyland") {
	///     assert_eq!(o.get(), &12);
	/// };
	/// ```
	fn get(&self) -> &Self::Item;

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
	fn get_mut(&mut self) -> &mut Self::Item;

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
	fn into_mut(self) -> Self::ItemMut<'a>;

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
	fn insert(&mut self, value: Self::Item) -> Self::Item;

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
	fn remove(self) -> Self::Item {
		self.remove_entry().1
	}
}

/// A view into a vacant entry.
/// It is part of the [`Entry`] enum.
/// See also [`KeyVacantEntry`] for entries that own there key
pub trait VacantEntry<'a>: CollectionRef + CollectionMut + KeyedRef {
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
	fn insert(self, value: Self::Item) -> Self::ItemMut<'a>;
}

pub trait KeyVacantEntry<'a>: VacantEntry<'a> {
	/// Gets a reference to the key that would be used when inserting a value
	/// through the `VacantEntry`.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// assert_eq!(map.entry("poneyland").key(), &"poneyland");
	/// ```
	fn key(&self) -> &Self::Key;

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
	fn into_key(self) -> Self::Key;
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
/// 	let z: HashMapVacantEntry<_,_> = y;
/// }
/// ```
pub enum Entry<Occ, Vac> {
	/// An occupied entry.
	Occupied(Occ),
	/// A vacant entry.
	Vacant(Vac),
}

impl<'a, Occ, Vac> Entry<Occ, Vac>
where
	Occ: 'a + OccupiedEntry<'a>,
	Vac: 'a
		+ VacantEntry<
			'a,
			Key = Occ::Key,
			Item = Occ::Item,
			KeyRef = Occ::KeyRef<'a>,
			ItemRef = Occ::ItemRef<'a>,
			ItemMut = Occ::ItemMut<'a>,
		>,
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
	pub fn or_insert(self, default: Occ::Item) -> Occ::ItemMut<'a> {
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
	pub fn or_insert_with<F: FnOnce() -> Occ::Item>(self, default: F) -> Occ::ItemMut<'a> {
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
	///    .and_modify(|e| { *e += 1 })
	///    .or_insert(42);
	/// assert_eq!(map["poneyland"], 42);
	///
	/// map.entry("poneyland")
	///    .and_modify(|e| { *e += 1 })
	///    .or_insert(42);
	/// assert_eq!(map["poneyland"], 43);
	/// ```
	#[inline]
	pub fn and_modify<F>(self, f: F) -> Self
	where
		F: FnOnce(&mut Occ::Item),
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

impl<'a, Occ, Vac> Entry<Occ, Vac>
where
	Occ: 'a + OccupiedEntry<'a>,
	Vac: 'a
		+ KeyVacantEntry<
			'a,
			Key = Occ::Key,
			Item = Occ::Item,
			KeyRef = Occ::KeyRef<'a>,
			ItemRef = Occ::ItemRef<'a>,
			ItemMut = Occ::ItemMut<'a>,
		>,
{
	/// Returns a reference to this entry's key.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::entry_api::*;
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=u32> { HashMap::new() }
	/// let mut map = make_map();
	/// assert_eq!(map.entry("poneyland").key(), &"poneyland");
	/// ```
	#[inline]
	pub fn key(&self) -> &Occ::Key {
		match *self {
			Occupied(ref entry) => entry.key(),
			Vacant(ref entry) => entry.key(),
		}
	}

	#[inline]
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
	pub fn or_insert_with_key<F: FnOnce(&Occ::Key) -> Occ::Item>(
		self,
		default: F,
	) -> Occ::ItemMut<'a> {
		match self {
			Occupied(entry) => entry.into_mut(),
			Vacant(entry) => {
				let value = default(entry.key());
				entry.insert(value)
			}
		}
	}
}

impl<'a, Occ, Vac> Entry<Occ, Vac>
where
	Occ: 'a + OccupiedEntry<'a>,
	Vac: 'a
		+ VacantEntry<
			'a,
			Key = Occ::Key,
			Item = Occ::Item,
			KeyRef = Occ::KeyRef<'a>,
			ItemRef = Occ::ItemRef<'a>,
			ItemMut = Occ::ItemMut<'a>,
		>,
	Occ::Item: Default,
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
	pub fn or_default(self) -> Occ::ItemMut<'a> {
		match self {
			Occupied(entry) => entry.into_mut(),
			Vacant(entry) => entry.insert(Default::default()),
		}
	}
}

impl<'a, Occ, Vac> Debug for Entry<Occ, Vac>
where
	Occ: OccupiedEntry<'a> + Debug,
	Vac: VacantEntry<'a, Key = Occ::Key, Item = Occ::Item> + Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Vacant(ref v) => f.debug_tuple("Entry").field(v).finish(),
			Occupied(ref o) => f.debug_tuple("Entry").field(o).finish(),
		}
	}
}

#[cfg(feature = "raw_entry")]
pub struct RefOccupiedEntry<Occ>(pub(crate) Occ);

impl<'a, Occ: OccupiedEntry<'a>> AssociatedCollection for RefOccupiedEntry<Occ> {
	type Owner = Occ;
}

#[cfg(feature = "raw_entry")]
impl<'a, Occ: OccupiedEntry<'a>> OccupiedEntry<'a> for RefOccupiedEntry<Occ> {
	fn key(&self) -> &Self::Key {
		self.0.key()
	}

	fn remove_entry(self) -> (Self::Key, Self::Item) {
		self.0.remove_entry()
	}

	fn get(&self) -> &Self::Item {
		self.0.get()
	}

	fn get_mut(&mut self) -> &mut Self::Item {
		self.0.get_mut()
	}

	fn into_mut(self) -> Self::ItemMut<'a> {
		self.0.into_mut()
	}

	fn insert(&mut self, value: Self::Item) -> Self::Item {
		self.0.insert(value)
	}

	fn remove(self) -> Self::Item {
		self.0.remove()
	}
}

#[cfg(feature = "raw_entry")]
impl<'a, Occ: OccupiedEntry<'a>> Debug for RefOccupiedEntry<Occ>
where
	Occ::Key: Debug,
	Occ::Item: Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("RefOccupiedEntry")
			.field("key", self.key())
			.field("value", self.get())
			.finish_non_exhaustive()
	}
}

#[cfg(feature = "raw_entry")]
pub trait RawVacantEntry<'a>: Sized + KeyedRef + CollectionMut + CollectionRef {
	fn insert(self, key: Self::Key, value: Self::Item) -> (&'a mut Self::Key, Self::ItemMut<'a>);
}

#[cfg(feature = "raw_entry")]
pub struct RefVacantEntry<Q, Vac> {
	pub(crate) key: Q,
	pub(crate) raw: Vac,
}

impl<'a, Q: ?Sized, Vac: RawVacantEntry<'a>> AssociatedCollection for RefVacantEntry<&'a Q, Vac> {
	type Owner = Vac;
}

#[cfg(feature = "raw_entry")]
impl<'a, Q: ToOwned<Owned = Vac::Key> + ?Sized, Vac: RawVacantEntry<'a>> VacantEntry<'a>
	for RefVacantEntry<&'a Q, Vac>
where
	Vac::Key: 'a,
{
	fn insert(self, value: Self::Item) -> Self::ItemMut<'a> {
		self.raw.insert(self.key.to_owned(), value).1
	}
}

#[cfg(feature = "raw_entry")]
impl<'a, Q: ToOwned<Owned = Vac::Key> + Debug, Vac: RawVacantEntry<'a>> Debug
	for RefVacantEntry<&'a Q, Vac>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("RefVacantEntry")
			.field("key", self.key)
			.finish_non_exhaustive()
	}
}

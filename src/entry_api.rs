use std::borrow::Borrow;
use std::fmt;
use std::fmt::Debug;
use Entry::*;
use crate::{CollectionMut, CollectionRef, KeyedRef};


/// A view into an occupied entry.
/// It is part of the [`Entry`] enum.
pub trait OccupiedEntry<'a, C: CollectionMut+KeyedRef>: Sized {
	/// Gets a reference to the key in the entry.
	///
	/// # Examples
	///
	/// ```
	///use std::collections::HashMap;
	///use cc_traits::{EntryApi, entry_api::*};
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	/// assert_eq!(map.entry("poneyland").key(), &"poneyland");
	/// ```
	fn key(&self) -> C::KeyRef<'_>;

	/// Take the ownership of the key and value from the map.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::{EntryApi, entry_api::*, Get};
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
	/// use cc_traits::{EntryApi, entry_api::*};
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// map.entry("poneyland").or_insert(12);
	///
	/// if let Entry::Occupied(o) = map.entry("poneyland") {
	///     assert_eq!(o.get(), &12);
	/// };
	/// ```
	fn get(&self) -> C::ItemMut<'_>;

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
	/// use cc_traits::{EntryApi, entry_api::*};
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
	/// use cc_traits::{EntryApi, entry_api::*};
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
	/// use cc_traits::{EntryApi, entry_api::*};
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
	/// use cc_traits::{EntryApi, entry_api::*, Get};
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
/// See also [`KeyVacantEntry`] for entries that own there key
pub trait VacantEntry<'a, C: CollectionMut+KeyedRef>: Sized {
	/// Sets the value of the entry with the `VacantEntry`'s key,
	/// and returns a mutable reference to it.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::{EntryApi, entry_api::*};
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
}

pub trait KeyVacantEntry<'a, C: CollectionMut+KeyedRef>: VacantEntry<'a, C> {
	/// Gets a reference to the key that would be used when inserting a value
	/// through the `VacantEntry`.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::{EntryApi, entry_api::*};
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	/// assert_eq!(map.entry("poneyland").key(), &"poneyland");
	/// ```
	fn key(&self) -> C::KeyRef<'_>;

	/// Take ownership of the key.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::{EntryApi, entry_api::*};
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=i32> { HashMap::new() }
	/// let mut map = make_map();
	///
	/// if let Entry::Vacant(v) = map.entry("poneyland") {
	///     v.into_key();
	/// };
	/// ```
	fn into_key(self) -> C::Key;
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
/// use cc_traits::{EntryApi, entry_api::*};
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

impl<'a, Occ, Vac, C: CollectionMut+KeyedRef> Entry<Occ, Vac>
where
	Occ: OccupiedEntry<'a, C>,
	Vac: VacantEntry<'a, C>,
{
	/// Ensures a value is in the entry by inserting the default if empty, and returns
	/// a mutable reference to the value in the entry.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use std::ops::Index;
	/// use cc_traits::{EntryApi, entry_api::*};
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
	/// use cc_traits::{EntryApi};
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
	/// use cc_traits::{EntryApi};
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

impl<'a, Occ, Vac, C: CollectionMut+KeyedRef> Entry<Occ, Vac>
where
	Occ: OccupiedEntry<'a, C>,
	Vac: KeyVacantEntry<'a, C>,
{
	/// Returns a reference to this entry's key.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::HashMap;
	/// use cc_traits::{EntryApi};
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=u32> { HashMap::new() }
	/// let mut map = make_map();
	/// assert_eq!(map.entry("poneyland").key(), &"poneyland");
	/// ```
	#[inline]
	pub fn key(&self) -> C::KeyRef<'a> {
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
	/// use cc_traits::{EntryApi};
	///
	/// fn make_map() -> impl EntryApi<Key=&'static str,Item=usize> + Index<&'static str, Output=usize> { HashMap::new() }
	/// let mut map = make_map();
	///
	/// map.entry("poneyland").or_insert_with_key(|key| key.chars().count());
	///
	/// assert_eq!(map["poneyland"], 9);
	/// ```
	pub fn or_insert_with_key<F: FnOnce(C::KeyRef<'a>) -> C::Item>(self, default: F) -> C::ItemMut<'a> {
		match self {
			Occupied(entry) => entry.into_mut(),
			Vacant(entry) => {
				let value = default(entry.key());
				entry.insert(value)
			}
		}
	}
}

impl<'a, Occ, Vac, C: CollectionMut+KeyedRef> Entry<Occ, Vac>
where
	Occ: OccupiedEntry<'a, C>,
	Vac: VacantEntry<'a, C>,
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
	/// use cc_traits::{EntryApi};
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

impl<'a, Occ, Vac, C: CollectionMut+KeyedRef> Debug for Entry<Occ, Vac>
where
	Occ: OccupiedEntry<'a, C> + Debug,
	Vac: VacantEntry<'a, C> + Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Vacant(v) => f.debug_tuple("Entry").field(v).finish(),
			Occupied(o) => f.debug_tuple("Entry").field(o).finish(),
		}
	}
}

#[cfg(feature = "raw_entry")]
pub struct RefOccupiedEntry<Occ>(pub(crate) Occ);

#[cfg(feature = "raw_entry")]
impl<'a, Occ: OccupiedEntry<'a, C>, C: CollectionMut+KeyedRef+CollectionRef> OccupiedEntry<'a, C> for RefOccupiedEntry<Occ> {

	fn key(&self) -> C::KeyRef<'_> {
		self.0.key()
	}

	fn remove_entry(self) -> (C::Key, C::Item) {
		self.0.remove_entry()
	}

	fn get(&self) -> C::ItemRef<'_> {
		self.0.get()
	}

	fn get_mut(&mut self) -> C::ItemMut<'_> {
		self.0.get_mut()
	}

	fn into_mut(self) -> C::ItemMut<'a> {
		self.0.into_mut()
	}

	fn insert(&mut self, value: C::Item) -> C::Item {
		self.0.insert(value)
	}

	fn remove(self) -> C::Item {
		self.0.remove()
	}
}

#[cfg(feature = "raw_entry")]
impl<'a, Occ: OccupiedEntry<'a, C>, C: CollectionMut+KeyedRef+CollectionRef> Debug for RefOccupiedEntry<Occ>
where C::KeyRef<'a>: Debug, C::ItemRef<'a>: Debug {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("RefOccupiedEntry")
			.field("key", self.key())
			.field("value", self.get())
			.finish_non_exhaustive()
	}
}

#[cfg(feature = "raw_entry")]
pub trait RawVacantEntry<'a, C: CollectionMut+KeyedRef>: Sized {
	fn insert(self, key: C::Key, value: C::Item) -> (C::KeyRef<'a>, C::ItemMut<'a>);
}

#[cfg(feature = "raw_entry")]
pub struct RefVacantEntry<Q, Vac> {
	pub(crate) key: Q,
	pub(crate) raw: Vac,
}

#[cfg(feature = "raw_entry")]
impl<'a, 'b, Q: ?Sized, Vac: RawVacantEntry<'a, C>, C: CollectionMut+KeyedRef> VacantEntry<'a, C>
	for RefVacantEntry<&'b Q, Vac>
where C::Key: Borrow<Q>
{

	fn insert(self, value: C::Item) -> C::ItemMut<'a> {
		self.raw.insert(self.key.to_owned(), value).1
	}
}

#[cfg(feature = "raw_entry")]
impl<'a, 'b, Q: Debug, Vac: RawVacantEntry<'a, C>, C: CollectionMut+KeyedRef> Debug
	for RefVacantEntry<&'b Q, Vac>
where C::Key: Borrow<Q>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("RefVacantEntry")
			.field("key", self.key)
			.finish_non_exhaustive()
	}
}

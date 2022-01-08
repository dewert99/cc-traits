use crate::{
	Clear, Collection, CollectionMut, CollectionRef, Entry, EntryApi, Get, GetKeyValue, GetMut,
	Iter, KeyVacantEntry, Keyed, KeyedRef, Len, MapInsert, MapIter, MapIterMut, OccupiedEntry,
	Remove, VacantEntry,
};
use std::{
	borrow::Borrow,
	collections::{hash_map, HashMap},
	hash::{BuildHasher, Hash},
};
use std::default::Default;
use std::marker::PhantomData;

impl<K, V, S: BuildHasher> Collection for HashMap<K, V, S> {
	type Item = V;
}

impl<K, V, S: BuildHasher> CollectionRef for HashMap<K, V, S> {
	type ItemRef<'a>
	where
		Self: 'a,
	= &'a V;

	crate::covariant_item_ref!();
}

impl<K, V, S: BuildHasher> CollectionMut for HashMap<K, V, S> {
	type ItemMut<'a>
	where
		Self: 'a,
	= &'a mut V;

	crate::covariant_item_mut!();
}

impl<K, V, S: BuildHasher> Keyed for HashMap<K, V, S> {
	type Key = K;
}

impl<K, V, S: BuildHasher> KeyedRef for HashMap<K, V, S> {
	type KeyRef<'a>
	where
		Self: 'a,
	= &'a K;

	crate::covariant_key_ref!();
}

impl<K, V, S: BuildHasher> Len for HashMap<K, V, S> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<'a, Q, K: Hash + Eq, V, S: BuildHasher> Get<&'a Q> for HashMap<K, V, S>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get(&self, key: &'a Q) -> Option<&V> {
		self.get(key)
	}
}

impl<'a, Q, K: Hash + Eq, V, S: BuildHasher> GetMut<&'a Q> for HashMap<K, V, S>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get_mut(&mut self, key: &'a Q) -> Option<&mut V> {
		self.get_mut(key)
	}
}

impl<'a, Q, K: Hash + Eq, V, S: BuildHasher> GetKeyValue<&'a Q> for HashMap<K, V, S>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn get_key_value(&self, key: &'a Q) -> Option<(&K, &V)> {
		self.get_key_value(key)
	}
}

impl<K: Hash + Eq, V, S: BuildHasher> MapInsert<K> for HashMap<K, V, S> {
	type Output = Option<V>;

	#[inline(always)]
	fn insert(&mut self, key: K, value: V) -> Option<V> {
		self.insert(key, value)
	}
}

impl<'a, Q, K: Hash + Eq, V, S: BuildHasher> Remove<&'a Q> for HashMap<K, V, S>
where
	K: Borrow<Q>,
	Q: Hash + Eq + ?Sized,
{
	#[inline(always)]
	fn remove(&mut self, key: &'a Q) -> Option<V> {
		self.remove(key)
	}
}

impl<K, V, S: BuildHasher> Clear for HashMap<K, V, S> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<K, V, S: BuildHasher> Iter for HashMap<K, V, S> {
	type Iter<'a>
	where
		Self: 'a,
	= std::collections::hash_map::Values<'a, K, V>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.values()
	}
}

impl<K, V, S: BuildHasher> MapIter for HashMap<K, V, S> {
	type Iter<'a>
	where
		Self: 'a,
	= std::collections::hash_map::Iter<'a, K, V>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<K, V, S: BuildHasher> MapIterMut for HashMap<K, V, S> {
	type IterMut<'a>
	where
		Self: 'a,
	= std::collections::hash_map::IterMut<'a, K, V>;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}

/// A thin wrapper around a [`hashmap::OccupiedEntry`] that keeps it's hasher as phantom data
/// This is required so that it's owner can be HashMap<K, V, S> which is required for HashMap<K, V, S> to implement EntryApi
pub struct OccupiedEntryS<'a, K, V, S: 'a>(pub hash_map::OccupiedEntry<'a, K, V>, PhantomData<S>);

impl<'a, K, V, S: 'a + BuildHasher> OccupiedEntry<'a> for OccupiedEntryS<'a, K, V, S> {
	type Owner = HashMap<K, V, S>;

	#[inline(always)]
	fn key(&self) -> &K {
		hash_map::OccupiedEntry::key(&self.0)
	}

	#[inline(always)]
	fn remove_entry(self) -> (K, V) {
		hash_map::OccupiedEntry::remove_entry(self.0)
	}

	#[inline(always)]
	fn get(&self) -> &V {
		hash_map::OccupiedEntry::get(&self.0)
	}

	#[inline(always)]
	fn get_mut(&mut self) -> &mut V {
		hash_map::OccupiedEntry::get_mut(&mut self.0)
	}

	#[inline(always)]
	fn into_mut(self) -> &'a mut V {
		hash_map::OccupiedEntry::into_mut(self.0)
	}

	#[inline(always)]
	fn insert(&mut self, value: V) -> V {
		hash_map::OccupiedEntry::insert(&mut self.0, value)
	}

	#[inline(always)]
	fn remove(self) -> V {
		hash_map::OccupiedEntry::remove(self.0)
	}
}

pub struct VacantEntryS<'a, K, V, S: 'a>(pub hash_map::VacantEntry<'a, K, V>, PhantomData<S>);

impl<'a, K, V, S: 'a + BuildHasher> VacantEntry<'a> for VacantEntryS<'a, K, V, S> {
	type Owner = HashMap<K, V, S>;

	#[inline(always)]
	fn insert(self, value: V) -> &'a mut V {
		hash_map::VacantEntry::insert(self.0, value)
	}
}

impl<'a, K, V, S: BuildHasher> KeyVacantEntry<'a> for VacantEntryS<'a, K, V, S> {
	#[inline(always)]
	fn key(&self) -> & K {
		hash_map::VacantEntry::key(&self.0)
	}
	#[inline(always)]
	fn into_key(self) -> K {
		hash_map::VacantEntry::into_key(self.0)
	}
}

impl<K: Hash + Eq, V, S: BuildHasher> EntryApi for HashMap<K, V, S> {
	type Occ<'a>
	where
		Self: 'a,
	= OccupiedEntryS<'a, K, V, S>;
	type Vac<'a>
	where
		Self: 'a,
	= VacantEntryS<'a, K, V, S>;

	#[inline(always)]
	fn entry(&mut self, key: Self::Key) -> Entry<Self::Occ<'_>, Self::Vac<'_>> {
		match HashMap::entry(self, key) {
			hash_map::Entry::Occupied(o) => Entry::Occupied(OccupiedEntryS(o, Default::default())),
			hash_map::Entry::Vacant(v) => Entry::Vacant(VacantEntryS(v, Default::default())),
		}
	}
}

#[cfg(feature = "raw_entry")]
impl<'a, K, V, S: BuildHasher> OccupiedEntry<'a> for hash_map::RawOccupiedEntryMut<'a, K, V, S> {
	type Owner = HashMap<K, V, S>;

	#[inline(always)]
	fn key(&self) -> &K {
		hash_map::RawOccupiedEntryMut::key(self)
	}

	#[inline(always)]
	fn remove_entry(self) -> (K, V) {
		hash_map::RawOccupiedEntryMut::remove_entry(self)
	}

	#[inline(always)]
	fn get(&self) -> &V {
		hash_map::RawOccupiedEntryMut::get(self)
	}

	#[inline(always)]
	fn get_mut(&mut self) -> &mut V {
		hash_map::RawOccupiedEntryMut::get_mut(self)
	}

	#[inline(always)]
	fn into_mut(self) -> &'a mut V {
		hash_map::RawOccupiedEntryMut::into_mut(self)
	}

	#[inline(always)]
	fn insert(&mut self, value: V) -> V {
		hash_map::RawOccupiedEntryMut::insert(self, value)
	}

	#[inline(always)]
	fn remove(self) -> V {
		hash_map::RawOccupiedEntryMut::remove(self)
	}
}

#[cfg(feature = "raw_entry")]
impl<'a, K: Hash + Eq, V, S: BuildHasher> crate::RawVacantEntry<'a>
	for hash_map::RawVacantEntryMut<'a, K, V, S>
{
	type Owner = HashMap<K, V, S>;

	fn insert(self, key: K, value: V) -> (&'a K, &'a mut V) {
		let (k, v) = hash_map::RawVacantEntryMut::insert(self, key, value);
		(&*k, v)
	}
}

#[cfg(feature = "raw_entry")]
impl<Q: Hash + Eq + ToOwned<Owned = K> + ?Sized, K: Hash + Eq, V, S: BuildHasher> crate::EntryRefApi<Q>
	for HashMap<K, V, S>
	where K: Borrow<Q>
{
	type Occ<'a>
	where
		Self: 'a, Q: 'a
	= crate::RefOccupiedEntry<hash_map::RawOccupiedEntryMut<'a, K, V, S>>;
	type Vac<'a>
	where
		Self: 'a,
		Q: 'a,
	= crate::RefVacantEntry<&'a Q, hash_map::RawVacantEntryMut<'a, K, V, S>>;

	fn entry_ref<'a>(&'a mut self, key: &'a Q) -> Entry<Self::Occ<'a>, Self::Vac<'a>>
	where Q: 'a {
		let raw = self.raw_entry_mut();
		match raw.from_key(key) {
			hash_map::RawEntryMut::Occupied(occ) => Entry::Occupied(crate::RefOccupiedEntry(occ)),
			hash_map::RawEntryMut::Vacant(vac) => {
				Entry::Vacant(crate::RefVacantEntry { key, raw: vac })
			}
		}
	}
}

use crate::{Clear, Collection, CollectionMut, CollectionRef, Entry, EntryApi, Get, GetKeyValue, GetMut, Keyed, KeyedRef, KeyVacantEntry, Len, MapInsert, MapIter, MapIterMut, OccupiedEntry, Remove, VacantEntry};
use std::{borrow::Borrow, cmp::Ord, hash::Hash};

impl Collection for serde_json::Map<String, serde_json::Value> {
	type Item = serde_json::Value;
}

impl CollectionRef for serde_json::Map<String, serde_json::Value> {
	type ItemRef<'a> = &'a serde_json::Value;

	crate::covariant_item_ref!();
}

impl CollectionMut for serde_json::Map<String, serde_json::Value> {
	type ItemMut<'a> = &'a mut serde_json::Value;

	crate::covariant_item_mut!();
}

impl Keyed for serde_json::Map<String, serde_json::Value> {
	type Key = String;
}

impl KeyedRef for serde_json::Map<String, serde_json::Value> {
	type KeyRef<'a> = &'a String;

	crate::covariant_key_ref!();
}

impl Len for serde_json::Map<String, serde_json::Value> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl MapIter for serde_json::Map<String, serde_json::Value> {
	type Iter<'a> = serde_json::map::Iter<'a>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl MapIterMut for serde_json::Map<String, serde_json::Value> {
	type IterMut<'a> = serde_json::map::IterMut<'a>;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}

impl<'a, Q: ?Sized> Get<&'a Q> for serde_json::Map<String, serde_json::Value>
where
	String: Borrow<Q>,
	Q: Ord + Hash,
{
	#[inline(always)]
	fn get(&self, q: &'a Q) -> Option<&serde_json::Value> {
		self.get(q)
	}
}

impl<'a, Q: ?Sized> GetMut<&'a Q> for serde_json::Map<String, serde_json::Value>
where
	String: Borrow<Q>,
	Q: Ord + Hash,
{
	#[inline(always)]
	fn get_mut(&mut self, q: &'a Q) -> Option<&mut serde_json::Value> {
		self.get_mut(q)
	}
}

impl<'a, Q: ?Sized> GetKeyValue<&'a Q> for serde_json::Map<String, serde_json::Value>
where
	String: Borrow<Q>,
	Q: Ord + Hash,
{
	#[inline(always)]
	#[deny(unconditional_recursion)]
	fn get_key_value(&self, q: &'a Q) -> Option<(&String, &serde_json::Value)> {
		self.get_key_value(q)
	}
}

impl MapInsert<String> for serde_json::Map<String, serde_json::Value> {
	type Output = Option<serde_json::Value>;

	#[inline(always)]
	fn insert(&mut self, key: String, value: serde_json::Value) -> Option<serde_json::Value> {
		self.insert(key, value)
	}
}

impl<'a, Q: ?Sized> Remove<&'a Q> for serde_json::Map<String, serde_json::Value>
where
	String: Borrow<Q>,
	Q: Ord + Hash,
{
	#[inline(always)]
	fn remove(&mut self, key: &'a Q) -> Option<serde_json::Value> {
		self.remove(key)
	}
}

impl Clear for serde_json::Map<String, serde_json::Value> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<'a> OccupiedEntry<'a, serde_json::Map<String, serde_json::Value>> for serde_json::map::OccupiedEntry<'a> {

	#[inline(always)]
	fn key(&self) -> &String {
		serde_json::map::OccupiedEntry::key(self)
	}

	#[inline(always)]
	fn remove_entry(self) -> (String, serde_json::Value) {
		let key = self.key().clone();
		(key, self.remove()) // serde::json doesn't implement remove_entry so we use this instead
	}

	#[inline(always)]
	fn get(&self) -> &serde_json::Value {
		serde_json::map::OccupiedEntry::get(self)
	}

	#[inline(always)]
	fn get_mut(&mut self) -> &mut serde_json::Value {
		serde_json::map::OccupiedEntry::get_mut(self)
	}

	#[inline(always)]
	fn into_mut(self) -> &'a mut serde_json::Value {
		serde_json::map::OccupiedEntry::into_mut(self)
	}

	#[inline(always)]
	fn insert(&mut self, value: serde_json::Value) -> serde_json::Value {
		serde_json::map::OccupiedEntry::insert(self, value)
	}

	#[inline(always)]
	fn remove(self) -> serde_json::Value {
		serde_json::map::OccupiedEntry::remove(self)
	}
}

impl<'a> VacantEntry<'a, serde_json::Map<String, serde_json::Value>> for serde_json::map::VacantEntry<'a> {

	#[inline(always)]
	fn insert(self, value: serde_json::Value) -> &'a mut serde_json::Value {
		serde_json::map::VacantEntry::insert(self, value)
	}
}

impl<'a> KeyVacantEntry<'a, serde_json::Map<String, serde_json::Value>> for serde_json::map::VacantEntry<'a> {
	#[inline(always)]
	fn key(&self) -> &String {
		serde_json::map::VacantEntry::key(self)
	}

	#[inline(always)]
	fn into_key(self) -> String {
		self.key().clone() // serde::json doesn't implement into_key so we use this instead
	}
}

impl EntryApi for serde_json::Map<String, serde_json::Value> {
	type Occ<'a> = serde_json::map::OccupiedEntry<'a>;
	type Vac<'a> = serde_json::map::VacantEntry<'a>;

	#[inline(always)]
	fn entry(&mut self, key: Self::Key) -> Entry<'_, Self> {
		match serde_json::Map::entry(self, key) {
			serde_json::map::Entry::Occupied(o) => Entry::Occupied(o),
			serde_json::map::Entry::Vacant(v) => Entry::Vacant(v),
		}
	}
}

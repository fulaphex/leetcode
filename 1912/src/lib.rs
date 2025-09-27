use std::collections::{BTreeSet, HashMap};

struct MovieRentingSystem {
    price_map: HashMap<(i32, i32), i32>,
    // (price, shop, movie)
    rented: BTreeSet<(i32, i32, i32)>,
    // movie -> (price, shop)
    available: HashMap<i32, BTreeSet<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut price_map = HashMap::new();
        let mut available = HashMap::new();
        for entry in entries {
            assert_eq!(entry.len(), 3);
            let (shop, movie, price) = (entry[0], entry[1], entry[2]);
            price_map.insert((shop, movie), price);
            available
                .entry(movie)
                .or_insert(BTreeSet::new())
                .insert((price, shop));
        }

        Self {
            price_map: price_map,
            rented: BTreeSet::new(),
            available: available,
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        println!("searching for {}", movie);
        return self
            .available
            .get(&movie)
            .unwrap_or(&BTreeSet::new())
            .iter()
            .take(5)
            .map(|&(_price, shop)| shop)
            .collect();
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        println!("renting a movie {} from shop {}", movie, shop);
        // find the price
        let &price = self.price_map.get(&(shop, movie)).unwrap();
        // remove from available
        self.available
            .get_mut(&movie)
            .unwrap()
            .remove(&(price, shop));
        // add to rented
        self.rented.insert((price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        println!("dropping a movie {} to shop {}", movie, shop);
        // find the price
        let &price = self.price_map.get(&(shop, movie)).unwrap();
        // add to available
        self.available
            .get_mut(&movie)
            .unwrap()
            .insert((price, shop));
        // remove from rented
        self.rented.remove(&(price, shop, movie));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        println!("reporting rented movies");
        return self
            .rented
            .iter()
            .take(5)
            .map(|&(_price, shop, movie)| vec![shop, movie])
            .collect();
    }
}

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let entries = [
            [0, 1, 5],
            [0, 2, 6],
            [0, 3, 7],
            [1, 1, 4],
            [1, 2, 7],
            [2, 1, 5],
        ]
        .into_iter()
        .map(|x| Vec::from(x))
        .collect();
        let mut system = MovieRentingSystem::new(3, entries);
        assert_eq!(system.search(1), vec![1, 0, 2]);
        system.rent(0, 1);
        system.rent(1, 2);
        assert_eq!(system.report(), [[0, 1], [1, 2]]);
        system.drop(1, 2);
        assert_eq!(system.search(2), vec![0, 1]);
    }
}

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter::zip;

struct FoodRatings {
    ratings: HashMap<String, i32>,
    ratings_per_cuisine: HashMap<String, BTreeSet<(i32, String)>>,
    cuisine_by_food: HashMap<String, String>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let ratings_map = zip(foods.clone(), ratings.clone()).collect();
        let mut cuisine_by_food = HashMap::new();
        let mut ratings_per_cuisine = HashMap::new();
        for ((cuisine, food), &rating) in zip(zip(&cuisines, &foods), &ratings) {
            cuisine_by_food.insert(food.clone(), cuisine.clone());
            let cuisine_ratings = ratings_per_cuisine
                .entry(cuisine.clone())
                .or_insert(BTreeSet::new());
            cuisine_ratings.insert((-rating, food.clone()));
        }

        return Self {
            ratings: ratings_map,
            ratings_per_cuisine: ratings_per_cuisine,
            cuisine_by_food: cuisine_by_food,
        };
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let cuisine = self.cuisine_by_food.get(&food).unwrap();
        let cuisine_ratings = self.ratings_per_cuisine.get_mut(cuisine).unwrap();
        let rating = self.ratings.get(&food).unwrap();

        cuisine_ratings.remove(&(-*rating, food.clone()));
        cuisine_ratings.insert((-new_rating, food.clone()));

        *self.ratings.get_mut(&food).unwrap() = new_rating;
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let x = self
            .ratings_per_cuisine
            .get(&cuisine)
            .unwrap()
            .first()
            .unwrap();
        return x.1.clone();
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (foods, cuisines, ratings): (Vec<String>, Vec<String>, Vec<i32>) = (
            ["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"]
                .into_iter()
                .map(|x| String::from(x))
                .collect(),
            [
                "korean", "japanese", "japanese", "greek", "japanese", "korean",
            ]
            .into_iter()
            .map(|x| String::from(x))
            .collect(),
            vec![9, 12, 8, 15, 14, 7],
        );
        let mut ratings = FoodRatings::new(foods, cuisines, ratings);
        assert_eq!(
            ratings.highest_rated(String::from("korean")),
            String::from("kimchi")
        );
        assert_eq!(
            ratings.highest_rated(String::from("japanese")),
            String::from("ramen")
        );
        ratings.change_rating(String::from("sushi"), 16);
        assert_eq!(
            ratings.highest_rated(String::from("japanese")),
            String::from("sushi")
        );
        ratings.change_rating(String::from("ramen"), 16);
        assert_eq!(
            ratings.highest_rated(String::from("japanese")),
            String::from("ramen")
        );
    }
}

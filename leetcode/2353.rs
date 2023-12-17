/*
Question:
Design a food rating system that can do the following:
    Modify the rating of a food item listed in the system.
    Return the highest-rated food item for a type of cuisine in the system.

Implement the FoodRatings class:
    FoodRatings(String[] foods, String[] cuisines, int[] ratings) Initializes the system.
The food items are described by foods, cuisines and ratings, all of which have a length of n.
        foods[i] is the name of the ith food,
        cuisines[i] is the type of cuisine of the ith food, and
        ratings[i] is the initial rating of the ith food.
    void changeRating(String food, int newRating) Changes the rating of the food item with the name food.
    String highestRated(String cuisine) Returns the name of the food item that has the highest rating for the given type of cuisine.
        If there is a tie, return the item with the lexicographically smaller name.

Note that a string x is lexicographically smaller than string y if x comes before y in dictionary order,
that is, either x is a prefix of y, or if i is the first position such that x[i] != y[i], then x[i] comes before y[i] in alphabetic order.
*/

// Approach:
// 1) Create a struct FoodRatings with 3 fields:
//   - foods: HashMap<String, BTreeSet<(i32, String)>>
//   - ratings: HashMap<String, i32>
//   - cuisines: HashMap<String, String>
// 2) In the constructor, populate the fields by iterating over the input vectors.
// 3) To change the rating of a food, get the rating from the ratings HashMap,
//      remove the food from the BTreeSet in foods,
//      update the rating in ratings HashMap,
//      and insert the food back into the BTreeSet.
// 4) To get the highest rated food for a cuisine,
//      get the BTreeSet from foods HashMap
//      and return the first element of the BTreeSet.

use std::collections::{BTreeSet, HashMap};

struct FoodRatings {
    foods: HashMap<String, BTreeSet<(i32, String)>>,
    ratings: HashMap<String, i32>,
    cuisines: HashMap<String, String>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut c = HashMap::new();
        let mut f = HashMap::new();
        let mut r = HashMap::new();
        for i in 0..foods.len() {
            *c.entry(foods[i].clone()).or_insert(String::new()) = cuisines[i].clone();
            *r.entry(foods[i].clone()).or_insert(0) = ratings[i];
            f.entry(cuisines[i].clone())
                .or_insert(BTreeSet::new())
                .insert((-ratings[i], foods[i].clone()));
        }
        FoodRatings {
            cuisines: c,
            ratings: r,
            foods: f,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        match self.ratings.get_mut(&food) {
            Some(r) => {
                let f = self
                    .foods
                    .get_mut(self.cuisines.get(&food).unwrap())
                    .unwrap();
                let fd = (-r.clone(), food.clone());
                f.remove(&fd);
                *r = new_rating;
                f.insert((-new_rating, food.clone()));
            }
            None => (),
        }
    }

    fn highest_rated(&self, cuisine: String) -> String {
        match self.foods.get(&cuisine) {
            Some(c) => {
                if !c.is_empty() {
                    c.iter().next().unwrap().clone().1
                } else {
                    String::new()
                }
            }
            None => String::new(),
        }
    }
}

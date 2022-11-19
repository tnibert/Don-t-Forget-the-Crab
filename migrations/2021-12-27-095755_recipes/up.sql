CREATE TABLE recipes (
  id SERIAL PRIMARY KEY,
  recipe_name VARCHAR NOT NULL,
  notes VARCHAR,
  UNIQUE(recipe_name)
);

CREATE TABLE ingredients (
  id SERIAL PRIMARY KEY,
  recipe_id SERIAL,
  ingredient_name VARCHAR NOT NULL,
  amount FLOAT4 NOT NULL,
  unit VARCHAR NOT NULL,
  notes VARCHAR,
  CONSTRAINT fk_recipe
      FOREIGN KEY(recipe_id) 
      REFERENCES recipes(id)
);

--initialize with Thanksgiving recipes
INSERT INTO recipes (recipe_name, notes) VALUES ('Green Bean Casserole', 'https://www.mccormick.com/frenchs/recipes/salads-sides/frenchs-green-bean-casserole');
INSERT INTO recipes (recipe_name, notes) VALUES ('Cranberry Delight Salad', NULL);
INSERT INTO recipes (recipe_name, notes) VALUES ('Sweet Potato Casserole', NULL);
INSERT INTO recipes (recipe_name, notes) VALUES ('Corn Souffle Casserole', 'https://www.the-girl-who-ate-everything.com/corn-souffle/, http://www.thekitchenwhisperer.net/2015/01/10/baked-creamy-corn-casserole/');
INSERT INTO recipes (recipe_name, notes) VALUES ('Broccoli Casserole', 'https://www.dinneratthezoo.com/broccoli-casserole/');

--desserts
INSERT INTO recipes (recipe_name, notes) VALUES ('Pumpkin Pie', NULL);
INSERT INTO recipes (recipe_name, notes) VALUES ('Blackberry Pie', NULL);
INSERT INTO recipes (recipe_name, notes) VALUES ('Apple Pie', 'https://www.allrecipes.com/recipe/12682/apple-pie-by-grandma-ople/?internalSource=hub%20recipe&referringId=198&referringContentType=Recipe%20Hub');
INSERT INTO recipes (recipe_name, notes) VALUES ('Blackberry cobbler', 'https://www.allrecipes.com/recipe/65155/blackberry-cobbler-ii/?internalSource=recipe%20hub&referringId=198&referringContentType=Recipe%20Hub&clickId=cardslot%2064');
INSERT INTO recipes (recipe_name, notes) VALUES ('Pecan Pie', 'https://www.allrecipes.com/recipe/22544/pecan-pie-v/?internalSource=hub%20recipe&referringId=198&referringContentType=Recipe%20Hub');
INSERT INTO recipes (recipe_name, notes) VALUES ('Brown Butter Blondies', 'https://www.fifteenspatulas.com/featured-friday-brown-butter-toffee-blondies/?fbclid=IwAR0WXRvBS2UcIBckmrvxC_qQjmv8br_L6v06hAHNQyL9_V1f0LpiVkYhKR4');
INSERT INTO recipes (recipe_name, notes) VALUES ('Camembert baked with maple syrup, pecans and blueberries', 'https://www.sainsburysmagazine.co.uk/recipes/mains/camembert-baked-with-maple-syrup-pecans-and-blueberries?fbclid=IwAR2B5G3OK9B3j6fYigt_W9UWH7zkaYh4RIgohat8ewDnKhfNbdjp6n1gz_E');
INSERT INTO recipes (recipe_name, notes) VALUES ('Sticky Date Pudding', NULL);
INSERT INTO recipes (recipe_name, notes) VALUES ('Pavlova', NULL);

--add ingredients to recipes
--ingredients exist as constituent members of a larger set - recipe or grocery list, not in their own right, always have amount and unit
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Green Bean Casserole'), 'Campbells Condensed Cream of Mushroom Soup', 10.5, 'ounces');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Green Bean Casserole'), 'milk', 0.75, 'cups');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Green Bean Casserole'), 'black pepper', 0.125, 'teaspoon');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Green Bean Casserole'), 'can green beans', 2.0, '');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Green Bean Casserole'), 'frenchs crispy fried onions', 1.33, 'cups');

INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Cranberry Delight Salad'), 'hot water', 2.0, 'cups');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Cranberry Delight Salad'), 'pack lemon jello', 1.0, '');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Cranberry Delight Salad'), 'pack raspberry jello', 1.0, '');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit, notes) VALUES ((SELECT id FROM recipes where recipe_name='Cranberry Delight Salad'), 'cranberries', 2.0, 'cups', 'ground, can sub cranberry sauce');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit, notes) VALUES ((SELECT id FROM recipes where recipe_name='Cranberry Delight Salad'), 'pineapple', 0.75, 'cups', 'crushed');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit, notes) VALUES ((SELECT id FROM recipes where recipe_name='Cranberry Delight Salad'), 'celery', 2.0, 'cups', 'chopped');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Cranberry Delight Salad'), 'sugar', 1.5, 'cups');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Cranberry Delight Salad'), 'walnuts', 1.0, 'cups');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit, notes) VALUES ((SELECT id FROM recipes where recipe_name='Cranberry Delight Salad'), 'grapes', 1.33, 'cups', 'chopped');

INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'unsalted butter', 4.0, 'tbsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'large sweet potatoes', 4.0, '');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'milk', 0.5, 'cup');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'brown sugar', 0.25, 'cup');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'vanilla extract', 1.0, 'tsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'kosher salt', 0.5, 'tsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'eggs', 2.0, '');

INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'all-purpose flour', 1.0, 'cup');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'brown sugar', 1.0, 'cup');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'unsalted butter', 8.0, 'tbsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'kosher salt', 0.5, 'tsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit, notes) VALUES ((SELECT id FROM recipes where recipe_name='Sweet Potato Casserole'), 'walnuts', 1.5, 'cup', 'chopped');

INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Corn Souffle Casserole'), 'milk', 0.5, 'cup');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Corn Souffle Casserole'), 'heavy cream', 0.5, 'cup');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Corn Souffle Casserole'), 'unsalted butter', 2.0, 'tbsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Corn Souffle Casserole'), 'sugar', 1.5, 'tbsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Corn Souffle Casserole'), 'flour', 2.0, 'tbsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Corn Souffle Casserole'), 'salt', 1.0, 'tsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Corn Souffle Casserole'), 'corn kernels', 5.0, 'cups');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Corn Souffle Casserole'), 'eggs', 2.0, '');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit, notes) VALUES ((SELECT id FROM recipes where recipe_name='Corn Souffle Casserole'), 'asiago', 0.5, 'cup', 'shredded, optional');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit, notes) VALUES ((SELECT id FROM recipes where recipe_name='Corn Souffle Casserole'), 'Chives', 1.0, '', 'for garnish');

INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'broccoli florets', 6.0, 'cups');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit, notes) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'onion', 0.5, 'cups', 'finely diced');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'butter', 0.25, 'cups');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'flour', 0.25, 'cups');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'milk', 2.0, 'cups');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'cheddar cheese', 3.0, 'cups');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'cream cheese', 2.0, 'ounces');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'salt', 1.0, 'tsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'pepper', 0.5, 'tsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'smoked paprika', 1.0, 'tsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'garlic powder', 0.25, 'tsp');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'Ritz crackers', 1.0, 'cup');
INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit, notes) VALUES ((SELECT id FROM recipes where recipe_name='Broccoli Casserole'), 'parsley', 2.0, 'tbsp', 'chopped');
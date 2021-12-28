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
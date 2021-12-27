CREATE TABLE recipes (
  id SERIAL PRIMARY KEY,
  recipe_name VARCHAR NOT NULL,
  UNIQUE(recipe_name)
);

CREATE TABLE ingredients (
  id SERIAL PRIMARY KEY,
  recipe_id SERIAL,
  ingredient_name VARCHAR NOT NULL,
  amount FLOAT NOT NULL,
  unit VARCHAR NOT NULL,
  CONSTRAINT fk_recipe
      FOREIGN KEY(recipe_id) 
      REFERENCES recipes(id)
);
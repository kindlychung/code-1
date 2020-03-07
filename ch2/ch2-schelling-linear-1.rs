struct Individual {
  preference: f32,
  class: String,
}

enum House {
  Occupied(Individual),
  Vacant
}

type Community = Vec(House);

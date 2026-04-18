// conteúdo referente às entidades de lugares, municípios
// e estados e regiões.

Table places {
  id           int         [pk, increment]
  CEP          char(9)     [not null, unique]
  street       varchar(30) [not null]
  neighborhood int         [not null]
}

Table neighborhood {
  id      int         [pk, increment]
  nbhName varchar(30) [not null]
  city    int         [not null]
}

Table cities {
  id       int         [pk, increment]
  cityName varchar(50) [not null]
  estate   char(2)     [not null]
}

Table estates {
  ufId           char(2)     [pk]
  estateName     varchar(50) [not null, unique]
  region         int         [not null]
}

Table region {
  id         int         [pk, increment]
  regionName varchar(25) [not null, unique]
}

Ref: places.neighborhood > neighborhood.id
Ref: neighborhood.city > cities.id
Ref: cities.estate > estates.ufId
Ref: estates.region > region.id

// user related -----------------------------------------------------
Table users {
  pkId                   int         [pk, increment]
  userName               varchar(50) [not null, unique]
  userMail               varchar(50) [not null, unique]
  fkCategory             int         [not null]
  fkLatestCategoryChange int         [unique]
}

Table userCategories {
  pkId         int         [pk, increment]
  categoryName varchar(50) [not null, unique]
}

Table userCategoryChanges {
  pkId        int       [pk, increment]
  fkFrom      int       [not null]
  fkTo        int       [not null]
  atTimeStamp timestamp [not null]
}

// report related ---------------------------------------------------
Table reports {
  pkId                 int       [pk, increment]
  fkUserOwner          int       [not null]
  openedAt             timestamp [not null]
  fkRecord             int       [not null]
  fkStatus             int       [not null]
  fkLatestStatusChange int       [unique]
}

Table reportStatuses {
  pkId       int         [pk, increment]
  statusName varchar(50) [not null, unique]
}

Table reportStatusChanges {
  pkId        int       [pk, increment]
  fkFrom      int       [not null]
  fkTo        int       [not null]
  atTimeStamp timestamp [not null]
}

// record related ---------------------------------------------------
Table records {
  pkId                 int       [pk, increment]
  openedAt             timestamp [not null]
  fkStatus             int       [not null]
  fkLatestStatusChange int       [unique]
  fkPlace              int       [not null]
}

Table recordStatuses {
  pkId       int         [pk, increment]
  statusName varchar(50) [not null, unique]
}

Table recordStatusChanges {
  pkId        int       [pk, increment]
  fkFrom      int       [not null]
  fkTo        int       [not null]
  atTimeStamp timestamp [not null]
}

// place related ---------------------------------------------------
Table places {
  pkId           int         [pk, increment]
  CEP            char(9)     [not null, unique]
  streetName     varchar(35) [not null]
  fkNeighborhood int         [not null]
}

Table neighborhoods {
  pkId    int         [pk, increment]
  nbhName varchar(30) [not null]
  fkCity  int         [not null]
}

Table cities {
  pkId     int         [pk, increment]
  cityName varchar(50) [not null]
  fkState  char(2)     [not null]
}

Table states {
  pkUfId    char(2)     [pk]
  stateName varchar(50) [not null, unique]
  fkRegion  int         [not null]
}

Table regions {
  pkId       int         [pk, increment]
  regionName varchar(25) [not null, unique]
}

Ref: users.fkCategory             > userCategories.pkId
Ref: users.fkLatestCategoryChange > userCategoryChanges.pkId
Ref: userCategoryChanges.fkFrom   > userCategories.pkId
Ref: userCategoryChanges.fkTo     > userCategories.pkId

Ref: reports.fkUserOwner          > users.pkId
Ref: reports.fkRecord             > records.pkId
Ref: reports.fkStatus             > reportStatuses.pkId
Ref: reports.fkLatestStatusChange > reportStatusChanges.pkId
Ref: reportStatusChanges.fkFrom   > reportStatuses.pkId
Ref: reportStatusChanges.fkTo     > reportStatuses.pkId

Ref: records.fkStatus             > recordStatuses.pkId
Ref: records.fkLatestStatusChange > recordStatusChanges.pkId
Ref: records.fkPlace              > places.pkId
Ref: recordStatusChanges.fkFrom   > recordStatuses.pkId
Ref: recordStatusChanges.fkTo     > recordStatuses.pkId

Ref: places.fkNeighborhood > neighborhoods.pkId
Ref: neighborhoods.fkCity  > cities.pkId
Ref: cities.fkState        > states.pkUfId
Ref: states.fkRegion       > regions.pkId

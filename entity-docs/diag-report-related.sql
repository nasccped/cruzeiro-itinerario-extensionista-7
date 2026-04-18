// conteúdo referente às entidades de reports, fichas e seus status.

Table reports {
  id           int       [pk, increment]
  reportOwner  int       [not null]
  openedAt     timestamp [not null]
  placeRecord  int       [not null]
  reportStatus int       [not null]
}

Table placeRecords {
  id       int       [pk, increment]
  openedAt timestamp [not null]
  status   int       [not null]
  place    int       [not null]
}

Table reportStatuses {
  id         int         [pk, increment]
  statusName varchar(50) [not null, unique]
}

Table recordStatuses {
  id         int         [pk, increment]
  statusName varchar(50) [not null, unique]
}

Ref: reports.placeRecord > placeRecords.id
Ref: reports.reportStatus > reportStatuses.id
Ref: placeRecords.status > recordStatuses.id

namespace app.drwrite

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U64

@wasmbus( actorReceive: true )
service DoiService {
  version: "0.1",
  operations: [ AddDoi, FetchDoi, QueryEvents ]
}

operation AddDoi {
  input: DoiNode,
  output: DoiNode
}

structure DoiNode {
  @required
  team: String,
  @required
  doi: String,
  @required
  folder: String,

  by_user: String,

  publisher: String,

  pub_year: U64,
  pub_month: U64,
  pub_day: U64,
  pol_year: U64,
  pol_month: U64,
  pol_day: U64,
  create_year: U64,
  create_month: U64,
  create_day: U64,

  @required
  titles: Titles,

  containers: Titles,

  summary: String,

  typ: String,

  authors: Names,
  editors: Names,

  links: Links
}

list Titles {
  member: String
}
list Names {
  member: Name
}
structure Name {
  suffix: String,
  given: String,
  family: String,
  prefix: String,
  name: String
}

list Links {
  member: Link
}
structure Link {
  @required
  url: String,
  ctype: String,
  cversion: String,
  app: String
}

operation FetchDoi {
  input: DoiRequest,
  output: DoiAck
}

operation FetchDoi {
  input: DoiRequest,
  output: DoiAck
}

structure DoiRequest {
  @required
  uid: String,
  user: String, 
  @required
  folder: String,
  @required
  doi: String
}
structure DoiAck {
    timeslot: U64
}

operation QueryEvents {
    input: TimeslotRequest,
    output: EventList
}

structure TimeslotRequest {
    timeslot: U64
}
structure DoiEvent {
    @required
    uid: String,
    user: String,
    folder: String,
    @required
    doi: String,
    raw: U64,
    prs: U64
}

list EventList {
  member: DoiEvent
}
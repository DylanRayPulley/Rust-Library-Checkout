Encapsulation
  Member and Catalog do not expose their internal collections directly.
  Borrowed items and catalog items can only be modified through the methods borrow, return_item, add, get, and details_for.

Use of Inheritance
  Book and Dvd both implement a shared Item trait.
  The Item trait defines common functions such as id, title, and days_allowed, but individual types have their own days_allowed value

Use of Composition
  Catalog stores a list of items, and Member stores a list of borrowed item IDs.
  These lists are part of each struct’s internal state, and only the struct’s own functions change them.

Polymorphism
  Any type that implements Item can be used wherever an Item is expected.
  item.days_allowed() can be called on an item without knowing which type it is, and Rust will run the correct version for that item.

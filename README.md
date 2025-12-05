Encapsulation:
  Member and Catalog both dont expose their collections directly and instead rely their own functions to edit items they contain 
Use of Inheritance
  Book and Dvd use inheritance as they are both classes that extend from Item
Use of Composition
  Catalog is a class that can hold a collection of Item objects
  Member holds a collection of item IDs

Polymorphism
  The subclasses created off of Item can all be referenced in the same location any other item class would be referenced, such as Dvd where Book is used

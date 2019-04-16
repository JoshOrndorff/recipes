# Economic Security

* modules are indepdent and should not have external dependencies
* they optimize for performance and trade composability

* do all the checks before all the function calls
* make sure a panic does not and cannot occur before a function change
    * use the qed with `.expect()` to really show that this is done correctly
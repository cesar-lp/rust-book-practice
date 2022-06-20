fn cook_order() {}

fn fix_incorrect_order() {
    cook_order();
    super::super::deliver_order();
}

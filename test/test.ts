#include <stdio.h>
#include "../src/backend.h"

// Remove
#test test_create_cell
    PointList* snake = create_cell(2, 3);
    fail_unless(snake->x == 2);
    fail_unless(snake->y == 3);

// Remove
#test test_create_random_cell
    PointList* cell1 = create_random_cell(20, 10);
    PointList* cell2 = create_random_cell(20, 10);
    fail_unless(!(cell1->x == cell2->x && cell1->y == cell2->y));
    fail_unless(cell1->x < 20);
    fail_unless(cell2->x < 20);
    fail_unless(cell1->y < 10);
    fail_unless(cell2->y < 10);

// Remove
#test test_list_contains_true
    PointList* cell = create_cell(1, 2);
    PointList* list = create_cell(2, 2);
    list->next = create_cell(1, 2);
    fail_unless(list_contains(cell, list));

// Remove
#test test_list_contains_false
    PointList* cell = create_cell(4, 2);
    PointList* list = create_cell(2, 2);
    list->next = create_cell(1, 2);
    fail_unless(!list_contains(cell, list));

// Remove
#test test_remove_from_list_true
    PointList* cell = create_cell(1, 2);
    PointList* list = create_cell(2, 2);
    list->next = create_cell(1, 2);
    fail_unless(remove_from_list(cell, &list));

// Remove
#test test_remove_from_list_false
    PointList* cell = create_cell(3, 2);
    PointList* list = create_cell(2, 2);
    list->next = create_cell(1, 2);
    fail_unless(!remove_from_list(cell, &list));


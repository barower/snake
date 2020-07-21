#include <stdio.h>
#include "../src/backend.h"

// Remove
#test test_create_cell
    PointList* snake = create_cell(2, 3);
    fail_unless(snake->x == 2);
    fail_unless(snake->y == 3);

#test test_create_board
    PointList* snake = create_cell(0, 0);
    Board* board = create_board(snake, NULL, 2, 3);
    fail_unless(board->xmax == 2);
    fail_unless(board->ymax == 3);
    fail_unless(board->snake != NULL);
    fail_unless(board->snake->x == 0);

// Remove
#test test_next_move_corner
    PointList* snake = create_cell(0, 0);
    Board* board = create_board(snake, NULL, 2, 2);
    fail_unless(next_move(board, UP) == NULL);
    fail_unless(next_move(board, LEFT) == NULL);
    fail_unless(next_move(board, DOWN) != NULL);
    fail_unless(next_move(board, RIGHT) != NULL);

// Remove
#test test_next_move_bottom
    PointList* snake = create_cell(0, 1);
    Board* board = create_board(snake, NULL, 2, 2);
    fail_unless(next_move(board, LEFT) == NULL);
    fail_unless(next_move(board, DOWN) == NULL);
    fail_unless(next_move(board, UP) != NULL);
    fail_unless(next_move(board, RIGHT) != NULL);

// Remove
#test test_next_move_left
    PointList* snake = create_cell(1, 2);
    Board* board = create_board(snake, NULL, 4, 4);
    PointList* moved = next_move(board, LEFT);
    fail_unless(moved->x == 0);
    fail_unless(moved->y == 2);

// Remove
#test test_next_move_up
    PointList* snake = create_cell(2, 2);
    Board* board = create_board(snake, NULL, 4, 4);
    PointList* moved = next_move(board, UP);
    fail_unless(moved->x == 2);
    fail_unless(moved->y == 1);

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

#test test_move_snake_with_food
    PointList* snake = create_cell(1, 2);
    PointList* foods = create_cell(2, 2);
    Board* board = create_board(snake, foods, 10, 10);
    move_snake(board, RIGHT);
    fail_unless(board->snake->x == 2);
    fail_unless(board->snake->y == 2);
    fail_unless(board->snake->next != NULL);
    fail_unless(board->snake->next->x == 1);
    fail_unless(board->snake->next->y == 2);

    fail_unless(board->foods->x != 2 || board->foods->y != 2);

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


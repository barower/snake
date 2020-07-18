#include "backend.h"
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>

enum Status move_snake(Board* board, enum Direction dir) {
  // Create a new beginning. Check boundaries.
  PointList* beginning = next_move(board, dir);
  if (beginning == NULL) {
    return FAILURE;
  }

  // If we've gone backwards, don't do anything
  if (board->snake->next && is_same_place(beginning, board->snake->next)) {
    beginning->next = NULL;
    free(beginning);
    return SUCCESS;
  }

  // Check for collisions
  if (list_contains(beginning, board->snake)) {
    return FAILURE;
  }

  // Check for food
  if (list_contains(beginning, board->foods)) {
    // Attach the beginning to the rest of the snake;
    beginning->next = board->snake;
    board->snake = beginning;
    remove_from_list(beginning, &(board->foods));
    add_new_food(board);

    return SUCCESS;
  }

  // Attach the beginning to the rest of the snake
  beginning->next = board->snake;
  board->snake = beginning;


  // Cut off the end
  PointList* end = board->snake;
  while(end->next->next) {
    end = end->next;
  }
  free(end->next);
  end->next = NULL;

  return SUCCESS;
}


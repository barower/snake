#include "backend.h"
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>

/*
 * Removes from the list or returns false
 */
bool remove_from_list(PointList* elt, PointList** list) {
  PointList *currP, *prevP;
  prevP = NULL;

  for (currP = *list;
      currP != NULL;
      prevP = currP, currP = currP->next) {
    if (is_same_place(currP, elt)) {
      if (prevP == NULL) {
        *list = currP->next;
      } else {
        prevP->next = currP->next;
      }
      free(currP);
      return true;
    }
  }
  return false;
}

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

PointList* next_move(Board* board, enum Direction dir) {
  PointList* snake = board->snake;
  int new_x = snake->x;
  int new_y = snake->y;
  switch(dir) {
    case UP:
      new_y = snake->y - 1;
      break;
    case DOWN:
      new_y = snake->y + 1;
      break;
    case LEFT:
      new_x = snake->x - 1;
      break;
    case RIGHT:
      new_x = snake->x + 1;
      break;
  }
  if (new_x < 0 || new_y < 0 || new_x >= board->xmax || new_y >= board->ymax) {
    return NULL;
  } else {
    return create_cell(new_x, new_y);
  }
}


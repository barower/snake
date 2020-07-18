#include <stdbool.h>

enum Direction { UP, DOWN, LEFT, RIGHT };
enum Status { SUCCESS, FAILURE };
struct PointList {
  int x;
  int y;
  struct PointList* next;
};

typedef struct PointList PointList;

typedef struct {
  PointList* snake;
  PointList* foods;
  int xmax;
  int ymax;
} Board;

// Remove
extern bool is_same_place(PointList* cell1, PointList* cell2);
extern enum Status move_snake(Board* board, enum Direction dir);
// Remove
extern PointList* next_move(Board* board, enum Direction dir);
// Remove
extern PointList* create_cell(int x, int y);
// Remove
extern PointList* create_random_cell(int xmax, int ymax);
extern PointList* create_snake();
extern Board* create_board(PointList* foods, PointList* snake, int xmax, int ymax);
// Remove
extern bool list_contains(PointList* cell, PointList* list);
// Remove
extern bool remove_from_list(PointList* elt, PointList** list);
extern void add_new_food(Board* board);

#include <ncurses.h>
extern enum Direction get_next_move(enum Direction previous);
extern void display_points(PointList* snake, const chtype symbol);

#include <stdio.h>
#include <stdlib.h>

#include <linux/input.h>
#include <unistd.h>

// clang-format off
const struct input_event
syn       = {.type = EV_SYN , .code = SYN_REPORT   , .value = 0},
esc_up    = {.type = EV_KEY , .code = KEY_ESC      , .value = 0},
esc_down  = {.type = EV_KEY , .code = KEY_ESC      , .value = 1};
// clang-format on

int read_event(struct input_event *event) {
  return fread(event, sizeof(struct input_event), 1, stdin) == 1;
}

void write_event(const struct input_event *event) {
  if (fwrite(event, sizeof(struct input_event), 1, stdout) != 1)
    exit(EXIT_FAILURE);
}

int main() {
  int delay = 20000;

  struct input_event ev;
  enum { START, CAPSLOCK_HELD, CAPS_WITH_HJKL } state = START;

  setbuf(stdin, NULL), setbuf(stdout, NULL);
  struct input_event pending_event;

  while (read_event(&ev)) {
    if (ev.type == EV_MSC && ev.code == MSC_SCAN)
      continue;

    if (ev.type != EV_KEY && ev.type != EV_REL && ev.type != EV_ABS) {
      write_event(&ev);
      continue;
    }

    switch (state) {
    case START:
      if (ev.type == EV_KEY && ev.code == KEY_CAPSLOCK && ev.value)
        state = CAPSLOCK_HELD;
      else
        write_event(&ev);
      break;
    case CAPSLOCK_HELD:
      if (ev.type == EV_KEY && ev.code == KEY_CAPSLOCK && ev.value == 0) {
        write_event(&esc_down);
        usleep(delay);
        write_event(&esc_up);
        state = START;
        // printf("%s", "JUSTCAPS");
      } else if ((ev.value == 1) &&
                 (ev.code == KEY_H || ev.code == KEY_J || ev.code == KEY_K ||
                  ev.code == KEY_L ||
                  ev.code ==
                      KEY_CAPSLOCK)) { 
        if (ev.code != KEY_CAPSLOCK) {
          if (ev.code == KEY_H) {
            ev.code = KEY_LEFT;
          } else if (ev.code == KEY_J) {
            ev.code = KEY_DOWN;
          } else if (ev.code == KEY_K) {
            ev.code = KEY_UP;
          } else {
            ev.code = KEY_RIGHT;
          }
          write_event(&ev);
        } //
        state = CAPS_WITH_HJKL;
      }
      else
        write_event(&ev);
      break;
    case CAPS_WITH_HJKL:
      if (ev.code == KEY_H) {
        ev.code = KEY_LEFT;
        write_event(&ev);
        if (ev.value == 2) {
          pending_event = ev;
        }
      } else if (ev.code == KEY_J) {
        ev.code = KEY_DOWN;
        write_event(&ev);
        if (ev.value == 2) {
          pending_event = ev;
        }
      } else if (ev.code == KEY_K) {
        ev.code = KEY_UP;
        write_event(&ev);
        if (ev.value == 2) {
          pending_event = ev;
        }
      } else if (ev.code == KEY_L) {
        ev.code = KEY_RIGHT;
        write_event(&ev);
        if (ev.value == 2) {
          pending_event = ev;
        }
      } else { // caps lock up // if previous event remaining has repeat mode on turn it off
        if (pending_event.value == 2) {
          pending_event.value = 0;
          write_event(&pending_event);
        }
        state = START;
      }
      break;
    }
  }
}

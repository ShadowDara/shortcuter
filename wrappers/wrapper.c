// C Programm to just run a command

#include <stdlib.h>

int main(void) {
    // Hier den Befehl eintragen, den du ausführen willst:
    const char *cmd = "start";

    // Befehl ausführen
    int ret = system(cmd);

    return ret;
}

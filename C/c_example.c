#include <stdio.h>
#include <string.h>

#include "c_example.h"

#define NUM 10

static int id=0; // példányosításhoz
static int st_counter[NUM];
static char st_s[NUM][200];

int Example(char *s) {
    st_counter[id]=0;
    strncpy(st_s[id], s, sizeof(st_s[id]));
    return id++;
}

char *Get(int id) {
    static char s[210];
    sprintf(s, "%2d %s", st_counter[id]++, st_s[id]);
    return s;
}

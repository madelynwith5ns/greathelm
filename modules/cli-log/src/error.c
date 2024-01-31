#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
    if (argc < 2) {
        printf("Usage: %s <text>\n", argv[0]);
        return 1;
    }
    int len = 0;
    for (int i = 1; i < argc; i++) {
        len += strlen(argv[i]);
    }
    char *text = (char *)malloc(len + argc - 2);
    text[0] = '\0';
    for (int i = 1; i < argc; i++) {
        strcat(text, argv[i]);
        if (i < argc - 1) {
            strcat(text, " "); 
        }
    }
    char *layers = getenv("GREATHELM_EMBEDDED_LAYERS");
    if (layers != NULL) {
        for (int i = 0; i < atoi(layers); i++) {
            printf("\x1b[38;5;240m[\x1b[38;5;60mCHILD\x1b[38;5;240m] ");
        }
    }
    printf("\x1b[38;5;240m[\x1b[38;5;210mERROR\x1b[38;5;240m] \x1b[1;0m%s\n", text);
    free(text);
    return 0;
}

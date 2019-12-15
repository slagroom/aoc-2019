#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *read_line();

struct Graph;
struct Graph *new_graph(char *);
void free_graph(struct Graph *);
int add_node(struct Graph *, char *, char *);
void insert_node(struct Graph *, char *);
void print(struct Graph *graph, int indent);

struct Item;
struct Graph;
struct Item * new_item(char *tee, char *ter);
void free_item(struct Item *item);
struct Stack * new_stack();
void free_stack(struct Stack *stack);
void push(struct Stack *stack, struct Item *item);
struct Item * pop(struct Stack *stack);


char * read_line()
{
    char *line = NULL;
    size_t linecap = 0;
    ssize_t linelen = getline(&line, &linecap, stdin);

    if (linelen <= 0) { return NULL; }

    int i = linelen - 1;
    while (line[i] == '\r' || line[i] == '\n')
    {
        line[i] = '\0';
    }

    return line;
}


struct Graph
{
    char *name;
    struct Graph **nodes;
    size_t length;
    size_t capacity;
};

struct Graph * new_graph(char *name)
{
    struct Graph *graph = (struct Graph *)malloc(sizeof(struct Graph));
    if (graph == NULL)
    {
        perror("new_graph: failed to allocate memory for Graph");
        exit(1);
    }

    size_t name_len = strlen(name);
    graph->name = (char *)malloc((name_len + 1) * sizeof(char));
    if (graph->name == NULL)
    {
        perror("new_graph: failed to allocate memory for Graph.name");
        exit(1);
    }

    strcpy(graph->name, name);
    graph->name[name_len] = '\0';
    graph->nodes = NULL;
    graph->length = 0;
    graph->capacity = 0;
    return graph;
}

void free_graph(struct Graph *graph)
{
    free(graph->name);

    if (graph->nodes != NULL)
    {
        for (int i = 0; i < graph->length; i++)
        {
            free_graph(graph->nodes[i]);
        }
        free(graph->nodes);
    }

    free(graph);
}

int add_node(struct Graph *graph, char *child, char *parent)
{
    if (strcmp(graph->name, parent) == 0)
    {
        insert_node(graph, child);
        return 1;
    }

    for (int i = 0; i < graph->length; i++)
    {
        if (add_node(graph->nodes[i], child, parent))
        {
            return 1;
        }
    }
    
    return 0;
}

void insert_node(struct Graph *graph, char *add)
{
    if (graph->length == graph->capacity)
    {
        size_t new_capacity = graph->capacity == 0
            ? 16
            : graph->capacity * 2;

        void *new_buffer = realloc(graph->nodes, new_capacity * sizeof(struct Graph *));
        
        if (new_buffer == NULL)
        {
            perror("insert_node: failed to allocate memory for Graph.nodes");
            exit(1);
        }

        graph->nodes = new_buffer;
        graph->capacity = new_capacity;
    }

    graph->nodes[graph->length] = new_graph(add);
    graph->length++;
}

size_t parentages(struct Graph *graph, size_t depth)
{
    size_t total = graph->length * depth;
    for (int i = 0; i < graph->length; i++)
    {
        total += parentages(graph->nodes[i], depth + 1);
    }
    return total;
}

void print(struct Graph *graph, int indent)
{
    printf("%*s%s (len: %zu, cap: %zu)\r\n", indent*2, "", graph->name, graph->length, graph->capacity);
    for (int i = 0; i < graph->length; i++)
    {
        print(graph->nodes[i], indent + 1);
    }
}


struct Item
{
    char *tee;
    char *ter;
};

struct Stack
{
    struct Item **data;
    size_t size;
    size_t capacity;
};

struct Item * new_item(char *tee, char *ter)
{
    struct Item *item = (struct Item *)malloc(sizeof(struct Item));
    if (item == NULL)
    {
        perror("new_item: failed to allocate memory for Item");
        exit(1);
    }

    size_t tee_len = strlen(tee);
    item->tee = (char *)malloc((tee_len + 1) * sizeof(char));
    if (item->tee == NULL)
    {
        perror("new_item: failed to allocate memory for Item.tee");
        exit(1);
    }

    size_t ter_len = strlen(ter);
    item->ter = (char *)malloc((ter_len + 1) * sizeof(char));
    if (item->ter == NULL)
    {
        perror("new_item: failed to allocate memory for Item.ter");
        exit(1);
    }

    strcpy(item->tee, tee);
    strcpy(item->ter, ter);

    return item;
}

void free_item(struct Item *item)
{
    free(item->tee);
    free(item->ter);
    free(item);
}

struct Stack * new_stack()
{
    struct Stack *stack = (struct Stack *)malloc(sizeof(struct Stack));
    if (stack == NULL)
    {
        perror("new_stack: failed to allocate memory for Stack");
        exit(1);
    }
    
    stack->data = NULL;
    stack->size = 0;
    stack->capacity = 0;
    return stack;
}

void free_stack(struct Stack *stack)
{
    for (int i = 0; i < stack->size; i++)
    {
        free(stack->data[i]);
    }

    if (stack->data != NULL) { free(stack->data); }

    free(stack);
}

void print_stack(struct Stack *stack)
{
    printf("{\r\n"
        "    \"size\": \"%zu\",\r\n"
        "    \"capacity\": \"%zu\",\r\n"
        "    \"items\": [\r\n", stack->size, stack->capacity);

    for (int i = 0; i < stack->size; i++)
    {
        struct Item *p = stack->data[i];

        printf("        { \"tee\": \"%s\", \"ter\": \"%s\" }%s\r\n",
            p->tee,
            p->ter,
            (i + 1 == stack->size) ? "" : ",");
    }

    printf("    ],\r\n}\r\n");
}

void push(struct Stack *stack, struct Item *item)
{
    if (stack->size == stack->capacity)
    {
        size_t new_capacity = stack->capacity == 0
            ? 16
            : stack->capacity * 2;

        void *new_buffer = realloc(stack->data, new_capacity * sizeof(struct Item *));
        
        if (new_buffer == NULL)
        {
            perror("push: failed to allocate memory for Stack.data");
            exit(1);
        }

        stack->data = new_buffer;
        stack->capacity = new_capacity;
    }

    stack->data[stack->size] = item;
    stack->size++;
}

struct Item * pop(struct Stack *stack)
{
    if (stack->size == 0) { return NULL; }
    stack->size--;
    struct Item *item = stack->data[stack->size];
    stack->data[stack->size] = NULL;
    return item;
}


int main(int argc, char **argv)
{
    struct Graph *graph = new_graph("COM");
    struct Stack *stack = new_stack();

    char *line;
    while ((line = read_line()) != NULL)
    {
        size_t len = strlen(line);
        char *delim = strchr(line, ')');
        if (delim == NULL)
        {
            fprintf(stderr, "invalid input '%s'\r\n", line);
            exit(1);
        }
        *delim = '\0';
        delim++;
        
        struct Item *item = new_item(line, delim);
        push(stack, item);

        free(line);
    }

    while (stack->size > 0)
    {
        struct Stack *deferred = new_stack();

        while (stack->size > 0)
        {
            struct Item *item = pop(stack);

            if (add_node(graph, item->ter, item->tee))
                free_item(item);
            else
                push(deferred, item);
        }

        free_stack(stack);
        stack = deferred;
    }

    printf("%zu\r\n", parentages(graph, 1));
    free_stack(stack);
    free_graph(graph);
    return 0;
}
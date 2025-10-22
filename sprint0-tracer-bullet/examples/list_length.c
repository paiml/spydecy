/*
 * Sprint 0 Tracer Bullet - C Test Case
 *
 * Simplified version of CPython's list_length() implementation.
 * From: CPython/Objects/listobject.c
 *
 * This demonstrates the C code that implements Python's len() for lists.
 */

#include <stddef.h>

/* Simplified PyObject */
typedef struct {
    size_t ob_refcnt;
    void *ob_type;
} PyObject;

/* Simplified PyListObject */
typedef struct {
    PyObject ob_base;
    PyObject **ob_item;  /* Vector of pointers to list elements */
    size_t ob_size;      /* Number of items in the list */
} PyListObject;

/* Macro for getting size (like Py_SIZE in CPython) */
#define Py_SIZE(ob) (((PyListObject *)(ob))->ob_size)

/*
 * Get the length of a list object.
 * This is what CPython calls internally for len(list).
 */
static size_t
list_length(PyListObject *self)
{
    return Py_SIZE(self);
}

/*
 * Public API: PyList_Size
 * This is the function Python's len() actually calls for list objects.
 */
size_t
PyList_Size(PyObject *op)
{
    /* In real CPython, this would check if op is actually a list */
    /* For the tracer bullet, we simplify */
    return Py_SIZE(op);
}

/*
 * Test harness for the tracer bullet
 */
#ifdef TRACER_BULLET_TEST
#include <stdio.h>
#include <assert.h>

int main(void) {
    /* Create a mock list with 5 items */
    PyListObject test_list = {
        .ob_base = {0, NULL},
        .ob_item = NULL,
        .ob_size = 5
    };

    size_t result = list_length(&test_list);
    printf("Length of list is %zu\n", result);
    assert(result == 5);
    printf("✅ C test passed\n");

    return 0;
}
#endif

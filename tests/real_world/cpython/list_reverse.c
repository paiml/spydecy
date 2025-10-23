// Simplified CPython list_reverse implementation
static void list_reverse(PyListObject *self) {
    Py_ssize_t n = Py_SIZE(self);

    for (Py_ssize_t i = 0; i < n / 2; i++) {
        // Swap elements
        PyObject *tmp = self->ob_item[i];
        self->ob_item[i] = self->ob_item[n - i - 1];
        self->ob_item[n - i - 1] = tmp;
    }
}

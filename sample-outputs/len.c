static Py_ssize_t
list_length(PyListObject *self)
{
    return Py_SIZE(self);
}

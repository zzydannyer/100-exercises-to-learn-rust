# A dedicated `Client` type
# 专用的 `Client` 类型

All the interactions from the client side have been fairly low-level: you have to
manually create a response channel, build the command, send it to the server, and
then call `recv` on the response channel to get the response.
客户端一侧的所有交互都是相当底层的：你必须手动创建响应通道、构建命令、将其发送到服务器，然后在响应通道上调用 `recv` 来获取响应。

This is a lot of boilerplate code that could be abstracted away, and that's
exactly what we're going to do in this exercise.
这是许多可以抽象掉的样板代码，而这正是我们在这个练习中要做的事情。

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  createTicket,
  deleteTicket,
  fetchTickets,
  patchTicketStatus,
  statusLabel,
  type TicketStatus,
  type TicketVO,
} from "./ticket";

const tickets = ref<TicketVO[]>([]);
const statusText = ref("就绪");
const title = ref("");
const description = ref("");

async function refreshTickets(message: string) {
  const result = await fetchTickets();
  if (!result.ok) {
    statusText.value = result.message;
    return;
  }
  tickets.value = result.tickets;
  statusText.value = message;
}

async function onCreate() {
  const nextTitle = title.value.trim();
  if (!nextTitle) {
    statusText.value = "标题不能为空";
    return;
  }
  const result = await createTicket({
    title: nextTitle,
    description: description.value.trim(),
  });
  if (!result.ok) {
    statusText.value = result.message;
    return;
  }
  title.value = "";
  description.value = "";
  await refreshTickets("创建成功");
}

async function onPatch(id: number, status: TicketStatus) {
  const result = await patchTicketStatus(id, status);
  if (!result.ok) {
    statusText.value = result.message;
    return;
  }
  await refreshTickets(`#${id} 已更新`);
}

async function onDelete(id: number) {
  const result = await deleteTicket(id);
  if (!result.ok) {
    statusText.value = result.message;
    return;
  }
  await refreshTickets(`#${id} 已删除`);
}

onMounted(() => {
  refreshTickets("就绪");
});
</script>

<template>
  <div class="mx-auto max-w-3xl space-y-4 p-6">
    <header class="flex items-center justify-between gap-4">
      <div>
        <h1 class="text-3xl font-semibold tracking-tight">Ticket 控制台</h1>
        <p class="text-muted-foreground mt-1 text-sm">Axum · Tauri · Vue · shadcn-vue</p>
      </div>
      <Button variant="outline" @click="refreshTickets('列表已刷新')">刷新</Button>
    </header>

    <Card>
      <CardHeader>
        <CardTitle>新建工单</CardTitle>
        <CardDescription>创建后会立刻出现在下方列表</CardDescription>
      </CardHeader>
      <CardContent>
        <form class="space-y-4" @submit.prevent="onCreate">
          <div class="space-y-2">
            <Label for="title">标题</Label>
            <Input id="title" v-model="title" placeholder="例如：修复登录页" required />
          </div>
          <div class="space-y-2">
            <Label for="description">描述</Label>
            <Textarea
              id="description"
              v-model="description"
              rows="3"
              placeholder="补充细节（可选）"
            />
          </div>
          <Button type="submit">创建</Button>
        </form>
      </CardContent>
    </Card>

    <p class="text-muted-foreground min-h-5 text-sm">{{ statusText }}</p>

    <div class="space-y-3">
      <h2 class="text-lg font-medium">工单列表</h2>
      <Card v-if="tickets.length === 0">
        <CardContent class="text-muted-foreground py-10 text-center text-sm">
          还没有工单，先创建一个吧
        </CardContent>
      </Card>
      <Card v-for="ticket in tickets" :key="ticket.id">
        <CardHeader class="gap-3">
          <div class="flex items-center gap-2">
            <span class="text-muted-foreground text-sm font-semibold">#{{ ticket.id }}</span>
            <Badge :variant="statusLabel(ticket.status).variant">
              {{ statusLabel(ticket.status).text }}
            </Badge>
          </div>
          <CardTitle>{{ ticket.title }}</CardTitle>
          <CardDescription class="whitespace-pre-wrap">
            {{ ticket.description || "（无描述）" }}
          </CardDescription>
        </CardHeader>
        <CardFooter class="flex flex-wrap gap-2">
          <Button variant="outline" size="sm" @click="onPatch(ticket.id, 'in_progress')">
            进行中
          </Button>
          <Button variant="secondary" size="sm" @click="onPatch(ticket.id, 'done')">完成</Button>
          <Button variant="destructive" size="sm" @click="onDelete(ticket.id)">删除</Button>
        </CardFooter>
      </Card>
    </div>
  </div>
</template>

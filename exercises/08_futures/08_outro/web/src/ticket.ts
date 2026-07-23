export type TicketStatus = "to_do" | "in_progress" | "done";

export type TicketVO = {
  id: number;
  title: string;
  description: string;
  status: TicketStatus;
};

export type CreateTicketDTO = {
  title: string;
  description: string;
};

export type PatchTicketDTO = {
  status: TicketStatus;
};

export const API = "http://127.0.0.1:3000";

export function statusLabel(status: TicketStatus) {
  if (status === "to_do") return { text: "ToDo", variant: "default" as const };
  if (status === "in_progress") return { text: "进行中", variant: "secondary" as const };
  return { text: "完成", variant: "outline" as const };
}

export async function fetchTickets() {
  const res = await fetch(`${API}/tickets`);
  if (!res.ok) return { ok: false as const, message: `刷新失败 HTTP ${res.status}` };
  const tickets: TicketVO[] = await res.json();
  return { ok: true as const, tickets };
}

export async function createTicket(body: CreateTicketDTO) {
  const res = await fetch(`${API}/tickets`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  if (!res.ok) return { ok: false as const, message: `创建失败 HTTP ${res.status}` };
  return { ok: true as const };
}

export async function patchTicketStatus(id: number, status: TicketStatus) {
  const body: PatchTicketDTO = { status };
  const res = await fetch(`${API}/tickets/${id}`, {
    method: "PATCH",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  if (!res.ok) return { ok: false as const, message: `更新失败 HTTP ${res.status}` };
  return { ok: true as const };
}

export async function deleteTicket(id: number) {
  const res = await fetch(`${API}/tickets/${id}`, { method: "DELETE" });
  if (!res.ok) return { ok: false as const, message: `删除失败 HTTP ${res.status}` };
  return { ok: true as const };
}

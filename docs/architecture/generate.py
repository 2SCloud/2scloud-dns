#!/usr/bin/env python3
"""
Generate all scloud-dns architecture diagrams as SVG + PNG.

    python3 docs/architecture/generate.py

Diagrams are drawn from the real code in src/ (see README.md in this folder).
"""

import os
import diagramlib as d
from diagramlib import (
    box, edge, poly, panel, cylinder, text, textw, rrect, src, legend,
    title_block, footer, render,
    INK, MUTED, LINE, WHITE, PANEL, PANEL_LINE, BG,
    CLIENT, INGRESS, DECODE, CACHE, ROUTE, RESOLVE, EGRESS, SIDE, STUB, OK, STORE,
    GREEN, RED, AMBER, PURPLE, BLUE,
)

OUT = os.path.dirname(os.path.abspath(__file__))


# =====================================================================
# shared pipeline geometry (used by diagrams 02 and 03)
# =====================================================================
def positions():
    bw, bh = 130, 60
    sy = 322                      # spine box top  (center 352)
    return {
        "decoder":  (388, sy, bw, bh),
        "clook":    (560, sy, bw, bh),
        "disp":     (732, sy, bw, bh),
        "zone":     (912, 236, bw, bh),
        "resolver": (912, 408, bw, bh),
        "cwrite":   (1092, sy, bw, bh),
        "encoder":  (1272, sy, bw, bh),
        "sender":   (1452, sy, bw, bh),
        "listener": (206, 246, 132, 52),
        "tcp":      (206, 326, 132, 52),
        "doh":      (206, 406, 132, 52),
        "c_udp":    (44, 246, 120, 52),
        "c_tcp":    (44, 326, 120, 52),
        "c_doh":    (44, 406, 120, 52),
    }


def r(p):
    return d.anchors(*p)


# =====================================================================
# 01 - System context / deployment
# =====================================================================
def d01(c):
    W, H = 1300, 820
    title_block(c, W, "scloud-dns — System Context",
                "What talks to the server, what it produces, and where it runs")

    # clients
    panel(c, 40, 120, 300, 300, "DNS clients")
    box(c, 62, 165, 256, 58, "UDP client", ["dig / resolvers  ·  :53→:5353"], INGRESS)
    box(c, 62, 245, 256, 58, "TCP client", ["large answers, AXFR  ·  :5353"], INGRESS)
    box(c, 62, 325, 256, 58, "DoH client", ["browser / edge-gateway  ·  HTTP"], INGRESS)

    # server
    panel(c, 400, 120, 500, 560, "scloud-dns process")
    text(c, 420, 158, "Tokio multi-thread runtime · 8 worker threads", 12, color=MUTED)
    box(c, 424, 176, 452, 66, "Ingress listeners",
        ["UDP :5353 (SO_REUSEPORT) · TCP :5353 · DoH :8053"], DECODE)
    box(c, 424, 262, 452, 96, "13 worker types → many async tasks",
        ["LISTENER · DECODER · CACHE_LOOKUP · DISPATCHER",
         "ZONE_MANAGER · RESOLVER · CACHE_WRITER · ENCODER",
         "SENDER · CACHE_JANITOR · METRICS · TCP · DoH"], ROUTE)
    box(c, 424, 378, 220, 70, "In-memory cache",
        ["TTL records", "(planned)"], CACHE)
    box(c, 656, 378, 220, 70, "Authoritative zones",
        ["master / slave / inline", "(planned)"], RESOLVE)
    box(c, 424, 468, 452, 62, "config/config.json",
        ["listeners · workers · zones · forwarders · cache · ACLs · rate-limit"], SIDE)
    box(c, 424, 548, 452, 60, "Structured logging → OTLP",
        ["batched JSON logs → collector :4318"], SIDE)

    # externals
    panel(c, 960, 120, 300, 400, "Upstream / outputs")
    box(c, 982, 165, 256, 66, "Recursive upstreams",
        ["8.8.8.8 · 1.1.1.1 · forwarders", "(planned resolver)"], RESOLVE)
    box(c, 982, 250, 256, 60, "OTLP collector",
        ["localhost:4318 /v1/logs"], SIDE)
    box(c, 982, 330, 256, 60, "Prometheus / health",
        [":9153 · :8081  (planned)"], SIDE)
    box(c, 982, 410, 256, 66, "Admin API",
        ["127.0.0.1:8053  (planned)"], SIDE)

    # deployment
    panel(c, 960, 540, 300, 140, "Runs on")
    box(c, 982, 578, 118, 42, "Linux/mac/Win", None, CLIENT, title_size=11)
    box(c, 1112, 578, 126, 42, "Docker", None, CLIENT, title_size=11)
    box(c, 982, 628, 256, 40, "Kubernetes (k3s manifest)", None, CLIENT, title_size=11)

    cl = r((400, 120, 500, 560))
    for src_box in [(62, 165, 256, 58), (62, 245, 256, 58), (62, 325, 256, 58)]:
        a = r(src_box)
        edge(c, a["r"], (400, a["r"][1]), BLUE, width=2.4)
    # server -> externals
    edge(c, (900, 200), (982, 198), RESOLVE[0], label="resolve", width=2.2)
    edge(c, (900, 300), (982, 280), SIDE[0], label="logs", width=2.2, dashed=True)
    # reply back
    poly(c, [(900, 640), (930, 640), (930, 700), (200, 700), (200, 384)],
         d.EGRESS[0], dashed=True, label="DNS response", label_idx=2)

    legend(c, 60, 470,
           [("task / request flow", BLUE),
            ("response", d.EGRESS[0]),
            ("logs / side-effects", SIDE[0])])
    footer(c, W, H, "01 · generated from src/  ·  boxes marked (planned) are in the design but not yet implemented")


# =====================================================================
# 02 - Intended worker pipeline (how it SHOULD work)
# =====================================================================
def d02(c):
    W, H = 1800, 900
    P = positions()
    title_block(c, W, "Worker Pipeline — Intended Design",
                "How a query should flow: ingress → decode → cache → route → resolve → cache → encode → send")

    # clients + ingress
    box(c, *P["c_udp"], "UDP client", None, CLIENT, title_size=12)
    box(c, *P["c_tcp"], "TCP client", None, CLIENT, title_size=12)
    box(c, *P["c_doh"], "DoH client", None, CLIENT, title_size=12)
    L = box(c, *P["listener"], "LISTENER", ["UDP :5353"], INGRESS, title_size=13, badge="xN")
    T = box(c, *P["tcp"], "TCP_ACCEPTOR", ["TCP :5353"], INGRESS, title_size=12, badge="xN")
    Dh = box(c, *P["doh"], "DOH_ACCEPTOR", ["HTTP :8053"], INGRESS, title_size=12)

    De = box(c, *P["decoder"], "DECODER", ["bytes → DNSPacket"], DECODE, badge="xN")
    Cl = box(c, *P["clook"], "CACHE_LOOKUP", ["hit? / miss?"], CACHE, badge="xN")
    Di = box(c, *P["disp"], "QUERY_DISPATCHER", ["auth vs recursive"], ROUTE)
    Zo = box(c, *P["zone"], "ZONE_MANAGER", ["authoritative"], RESOLVE)
    Re = box(c, *P["resolver"], "RESOLVER", ["recursive / forward"], RESOLVE, badge="xN")
    Cw = box(c, *P["cwrite"], "CACHE_WRITER", ["store answer"], CACHE, badge="xN")
    En = box(c, *P["encoder"], "ENCODER", ["DNSPacket → bytes"], DECODE, badge="xN")
    Se = box(c, *P["sender"], "SENDER", ["reply to client"], EGRESS, badge="xN")

    # client -> ingress
    for cbox, ing in [("c_udp", L), ("c_tcp", T), ("c_doh", Dh)]:
        edge(c, r(P[cbox])["r"], ing["l"], BLUE, width=2.2)
    # ingress -> decoder (fan-in)
    for ing in (L, T, Dh):
        edge(c, ing["r"], De["l"], DECODE[0], width=2.0)

    edge(c, De["r"], Cl["l"], LINE)
    edge(c, Cl["r"], Di["l"], PURPLE, label="miss", label_bg=None, label_dy=-11)
    # hit shortcut over the top
    poly(c, [Cl["t"], (Cl["t"][0], 168), (En["t"][0], 168), En["t"]],
         AMBER, label="HIT → encode", label_idx=1)
    # dispatcher branches
    edge(c, Di["r"], Zo["l"], RESOLVE[0], label="auth")
    edge(c, Di["r"], Re["l"], RESOLVE[0], label="recurse")
    edge(c, Zo["r"], Cw["l"], RESOLVE[0])
    edge(c, Re["r"], Cw["l"], RESOLVE[0])
    edge(c, Cw["r"], En["l"], CACHE[0])
    edge(c, En["r"], Se["l"], DECODE[0])

    # reply back to clients across the very top
    poly(c, [Se["t"], (Se["t"][0], 116), (108, 116), (108, r(P["c_udp"])["t"][1])],
         EGRESS[0], dashed=True, label="DNS response", label_idx=1)

    # cache store
    Cy = cylinder(c, 726, 566, 158, 100, "in-memory", STORE, sub="cache (TTL)")
    edge(c, Cl["b"], (Cy["t"][0] - 20, Cy["t"][1]), STORE[0], dashed=True, label="read", label_dy=-4)
    poly(c, [Cw["b"], (Cw["b"][0], 620), (Cy["r"][0], 616)], STORE[0], dashed=True, label="write", label_idx=1)

    # side workers
    Ja = box(c, 360, 700, 168, 56, "CACHE_JANITOR", ["evict expired TTL"], SIDE, title_size=12)
    Me = box(c, 574, 700, 150, 56, "METRICS", ["batch → OTLP"], SIDE, title_size=12)
    Ot = box(c, 760, 700, 200, 56, "OTLP collector", [":4318 /v1/logs"], STORE, title_size=12)
    poly(c, [Ja["t"], (Ja["t"][0], 660), (Cy["b"][0] - 30, Cy["b"][1])], SIDE[0], dashed=True, label="evict", label_idx=1)
    edge(c, Me["r"], Ot["l"], SIDE[0], dashed=True)

    # captions
    text(c, 206, 214, "SO_REUSEPORT: kernel load-balances datagrams across N listeners", 11, italic=True, color=MUTED)
    text(c, 1080, 800, "Every edge is a bounded mpsc channel (cap 1024).", 11.5, color=MUTED)
    text(c, 1080, 820, "A Semaphore(512) permit rides with each task = backpressure.", 11.5, color=MUTED)
    text(c, 1080, 840, "xN = configurable instance count (config/config.json).", 11.5, color=MUTED)

    legend(c, 1080, 700,
           [("client / task flow", BLUE),
            ("cache access", STORE[0]),
            ("response (egress)", EGRESS[0]),
            ("side-effect / logs", SIDE[0])])
    footer(c, W, H, "02 · intended data-flow (channels_generation.rs wiring, with correct DNS semantics)")


# =====================================================================
# 03 - Current implementation (how it works TODAY)
# =====================================================================
def d03(c):
    W, H = 1800, 820
    P = positions()
    title_block(c, W, "Worker Pipeline — Current Implementation",
                "The graph is fully wired, but most stages are pass-through forwarders. Only one path answers end-to-end.")

    box(c, *P["c_udp"], "UDP client", None, CLIENT, title_size=12)
    box(c, *P["c_tcp"], "TCP client", None, CLIENT, title_size=12)
    box(c, *P["c_doh"], "DoH client", None, CLIENT, title_size=12)

    L = box(c, *P["listener"], "LISTENER", ["recv_from ✓"], OK, title_size=13)
    T = box(c, *P["tcp"], "TCP_ACCEPTOR", ["accept+frame ✓"], OK, title_size=12)
    Dh = box(c, *P["doh"], "DOH_ACCEPTOR", ["HTTP+reply reg ✓"], OK, title_size=12)

    De = box(c, *P["decoder"], "DECODER", ["logs + forwards", "no parse yet"], STUB)
    Cl = box(c, *P["clook"], "CACHE_LOOKUP", ["forwards tx[0]", "no cache"], STUB)
    Di = box(c, *P["disp"], "QUERY_DISPATCHER", ["forwarder", "bypassed"], STUB)
    Zo = box(c, *P["zone"], "ZONE_MANAGER", ["forwarder", "bypassed"], STUB)
    Re = box(c, *P["resolver"], "RESOLVER", ["forwarder", "bypassed"], STUB)
    Cw = box(c, *P["cwrite"], "CACHE_WRITER", ["forwarder", "no write"], STUB)
    En = box(c, *P["encoder"], "ENCODER", ["forwards raw", "no build"], STUB)
    Se = box(c, *P["sender"], "SENDER", ["DoH reply ✓", "UDP/TCP TODO"], CACHE)

    for cbox, ing in [("c_udp", L), ("c_tcp", T), ("c_doh", Dh)]:
        edge(c, r(P[cbox])["r"], ing["l"], BLUE, width=2.0)

    HOT = GREEN
    # The one path that answers today: DoH -> decoder -> clook -> cwrite(tx0) -> encoder -> sender -> reply.
    # cache_lookup forwards on tx[0] = cache_writer, so the router/resolver detour is skipped:
    # route the hot line over the top so it does not cut through the bypassed boxes.
    edge(c, Dh["r"], De["l"], HOT, width=3.2)
    edge(c, De["r"], Cl["l"], HOT, width=3.2)
    poly(c, [Cl["t"], (Cl["t"][0], 304), (Cw["t"][0], 304), Cw["t"]], HOT, width=3.0,
         label="tx[0] · skips router", label_idx=1)
    edge(c, Cw["r"], En["l"], HOT, width=3.2)
    edge(c, En["r"], Se["l"], HOT, width=3.2)

    # UDP/TCP ingress reach decoder too
    edge(c, L["r"], De["l"], LINE, width=1.8)
    edge(c, T["r"], De["l"], LINE, width=1.8)

    # wired-but-bypassed branches (greyed dashed)
    edge(c, Cl["r"], Di["l"], LINE, width=1.6, dashed=True, label="tx[1]", label_bg=None, label_dy=-11)
    edge(c, Di["r"], Zo["l"], LINE, width=1.6, dashed=True)
    edge(c, Di["r"], Re["l"], LINE, width=1.6, dashed=True)
    edge(c, Zo["r"], Cw["l"], LINE, width=1.6, dashed=True)
    edge(c, Re["r"], Cw["l"], LINE, width=1.6, dashed=True)

    # sender -> DoH reply via registry
    poly(c, [Se["b"], (Se["b"][0], 470), (Dh["b"][0], 470), Dh["b"]],
         HOT, width=2.6, dashed=True, label="reply_registry oneshot → DoH 200 OK", label_idx=1)
    # UDP/TCP reply gap
    xdrop = Se["r"][0] + 40
    edge(c, Se["r"], (xdrop, Se["r"][1]), RED, width=2.4, head=False)
    box(c, xdrop, Se["r"][1] - 26, 132, 52, "response", ["dropped (TODO)"], STUB, title_size=12)

    # side workers status
    Ja = box(c, 360, 566, 180, 56, "CACHE_JANITOR", ["no-op (returns Ok)"], STUB, title_size=12)
    Me = box(c, 580, 566, 160, 56, "METRICS", ["OTLP logger ✓"], OK, title_size=12)
    Cy = cylinder(c, 800, 552, 150, 92, "cache", STUB, sub="not implemented")

    legend(c, 1160, 566,
           [("works end-to-end today", GREEN),
            ("wired but bypassed", LINE),
            ("missing / TODO", STUB[0]),
            ("functional ingress", OK[0])])

    text(c, 360, 676, "Today a DoH query is accepted, its raw bytes are shuttled through the pass-through", 11.5, color=MUTED)
    text(c, 360, 694, "stages and echoed back verbatim (no real decode / cache / resolve / encode yet).", 11.5, color=MUTED)
    text(c, 360, 712, "UDP & TCP ingress work, but SENDER has no route back to the socket.", 11.5, color=MUTED)
    footer(c, W, H, "03 · current behaviour from src/workers/types/*.rs")


# =====================================================================
# 04 - Request lifecycle (sequence)
# =====================================================================
def d04(c):
    W, H = 1560, 940
    title_block(c, W, "Request Lifecycle — DoH query (the path that works today)",
                "One InFlightTask crosses several bounded channels; a oneshot returns the reply to the acceptor")

    lanes = [
        ("Client", CLIENT), ("DOH_ACCEPTOR", INGRESS), ("DECODER", DECODE),
        ("CACHE_LOOKUP", CACHE), ("CACHE_WRITER", CACHE), ("ENCODER", DECODE),
        ("SENDER", EGRESS), ("ReplyRegistry", STORE),
    ]
    xs = [100, 292, 484, 676, 868, 1052, 1240, 1436]
    top, bot = 150, 720
    X = {}
    for (name, cat), x in zip(lanes, xs):
        X[name] = x
        box(c, x - 66, top - 34, 132, 46, name, None, cat, title_size=11.5)
        src(c, PANEL_LINE); c.set_line_width(1.4); c.set_dash([4, 4])
        c.move_to(x, top + 12); c.line_to(x, bot); c.stroke(); c.set_dash([])

    n = [0]

    def msg(a, b, y, label, dashed=False, color=INK, ret=False):
        n[0] += 1
        x1, x2 = X[a], X[b]
        edge(c, (x1, y), (x2, y), color, width=2.2, dashed=dashed)
        mx = (x1 + x2) / 2
        # number badge
        rrect(c, mx - 9, y - 30, 18, 16, 8); src(c, color); c.fill()
        text(c, mx, y - 18, str(n[0]), 10.5, bold=True, color=WHITE, align="center")
        align = "center"
        text(c, mx, y - 6, label, 10.8, color=INK, align=align)

    def selfnote(a, y, label, color=MUTED):
        x = X[a]
        rrect(c, x + 10, y - 12, 190, 26, 8)
        src(c, "#fffbeb"); c.fill_preserve(); src(c, "#fde68a"); c.set_line_width(1.2); c.stroke()
        text(c, x + 18, y + 5, label, 10.2, color="#92400e")

    y = top + 60
    msg("Client", "DOH_ACCEPTOR", y, "POST /dns-query (dns-message)", color=BLUE); y += 46
    selfnote("DOH_ACCEPTOR", y, "acquire Semaphore(512) permit", ); y += 40
    msg("DOH_ACCEPTOR", "ReplyRegistry", y, "register(task_id) → oneshot rx", color=STORE[0]); y += 46
    msg("DOH_ACCEPTOR", "DECODER", y, "InFlightTask (mpsc 1024)", color=INK); y += 46
    msg("DECODER", "CACHE_LOOKUP", y, "forward", color=INK); y += 46
    msg("CACHE_LOOKUP", "CACHE_WRITER", y, "forward tx[0]", color=INK); y += 46
    msg("CACHE_WRITER", "ENCODER", y, "forward", color=INK); y += 46
    msg("ENCODER", "SENDER", y, "forward", color=INK); y += 46
    msg("SENDER", "ReplyRegistry", y, "take(task_id).send(payload)", color=GREEN); y += 46
    msg("ReplyRegistry", "DOH_ACCEPTOR", y, "oneshot resolves", dashed=True, color=GREEN); y += 46
    msg("DOH_ACCEPTOR", "Client", y, "200 OK application/dns-message", dashed=True, color=BLUE); y += 30

    selfnote("DOH_ACCEPTOR", y, "permit dropped when task completes")

    # timeout note
    rrect(c, 300, 748, 520, 54, 10); src(c, "#fef2f2"); c.fill_preserve()
    src(c, "#fecaca"); c.set_line_width(1.4); c.stroke()
    text(c, 318, 770, "Timeout guard", 11.5, bold=True, color=RED)
    text(c, 318, 790, "DOH_ACCEPTOR waits max 10s on the oneshot → 504 GATEWAY_TIMEOUT; if all channels full → 503.", 10.6, color="#7f1d1d")

    # UDP gap note
    rrect(c, 850, 748, 660, 54, 10); src(c, "#fff7ed"); c.fill_preserve()
    src(c, "#fed7aa"); c.set_line_width(1.4); c.stroke()
    text(c, 868, 770, "UDP / TCP path (today)", 11.5, bold=True, color=STUB[0])
    text(c, 868, 790, "LISTENER/TCP_ACCEPTOR build the same task, but reply_to=None → SENDER has no return route yet.", 10.6, color="#7c2d12")

    footer(c, W, H, "04 · doh_acceptor.rs · reply_registry.rs · sender.rs")


# =====================================================================
# 05 - Worker lifecycle & concurrency anatomy
# =====================================================================
def d05(c):
    W, H = 1500, 900
    title_block(c, W, "Worker Lifecycle & Concurrency Model",
                "SCloudWorker state machine, ordered startup, and the per-task backpressure mechanism")

    # -- left: state machine
    panel(c, 40, 110, 520, 740, "Worker state machine (WorkerState)")
    In = box(c, 205, 150, 190, 48, "INIT", None, SIDE, title_size=13)
    Id = box(c, 205, 250, 190, 48, "IDLE", None, OK, title_size=13)
    Bu = box(c, 205, 360, 190, 48, "BUSY", None, INGRESS, title_size=13)
    Pa = box(c, 410, 250, 120, 48, "PAUSED", None, CACHE, title_size=12)
    So = box(c, 205, 500, 190, 48, "STOPPING", None, STUB, title_size=12)
    Sd = box(c, 205, 600, 190, 48, "STOPPED", None, EGRESS, title_size=12)

    edge(c, In["b"], Id["t"], LINE, label="run()")
    edge(c, Id["b"], Bu["t"], GREEN, label="recv task")
    poly(c, [Bu["r"], (455, 384), (455, 320), Id["r"]], GREEN, label="task done", label_idx=1)
    edge(c, Id["r"], Pa["l"], AMBER, head=True)
    edge(c, (Pa["l"][0], Pa["l"][1] + 8), (Id["r"][0], Id["r"][1] + 8), AMBER, head=True)
    edge(c, Bu["b"], So["t"], RED, label="shutdown_requested")
    edge(c, So["b"], Sd["t"], RED)
    text(c, 60, 690, "ShutdownMode = GRACEFUL | IMMEDIATE", 11.5, color=MUTED)
    text(c, 60, 712, "Today run() sets INIT → IDLE, then loops on recv().", 11, italic=True, color=MUTED)
    text(c, 60, 730, "BUSY / PAUSED / STOPPING are defined for the design", 11, italic=True, color=MUTED)
    text(c, 60, 748, "but not yet driven by the worker loops.", 11, italic=True, color=MUTED)

    # -- right top: StartGate
    panel(c, 590, 110, 870, 250, "Ordered startup — StartGate (worker id 1,2,3, …)")
    Mn = box(c, 620, 175, 150, 56, "main()", ["spawn all"], SIDE, title_size=13)
    Ga = box(c, 810, 175, 150, 56, "StartGate", ["wait_turn(id)"], ROUTE, title_size=13)
    w1 = box(c, 1010, 160, 120, 46, "worker 1", None, OK, title_size=12)
    w2 = box(c, 1010, 216, 120, 46, "worker 2", None, INGRESS, title_size=12)
    w3 = box(c, 1010, 272, 120, 46, "worker N", None, CACHE, title_size=12)
    edge(c, Mn["r"], Ga["l"], LINE)
    for w, lbl in [(w1, "id=1"), (w2, "id=2"), (w3, "id=N")]:
        edge(c, Ga["r"], w["l"], PURPLE, label=lbl)
    text(c, 1160, 200, "each worker awaits its turn,", 11, color=MUTED)
    text(c, 1160, 218, "runs, then done() releases", 11, color=MUTED)
    text(c, 1160, 236, "the next id → deterministic", 11, color=MUTED)
    text(c, 1160, 254, "boot order.", 11, color=MUTED)

    # -- right bottom: per-task backpressure
    panel(c, 590, 385, 870, 465, "Per-task backpressure  (Semaphore → InFlightTask → mpsc channel)")
    Pr = box(c, 620, 470, 150, 66, "producer", ["LISTENER /", "acceptor"], INGRESS, title_size=13)
    Sem = box(c, 812, 470, 150, 66, "Semaphore", ["512 permits"], ROUTE, title_size=13)
    # channel slots
    chx, chy = 1010, 470
    text(c, chx, chy - 8, "mpsc channel (cap 1024)", 11, bold=True, color=MUTED)
    for i in range(6):
        rrect(c, chx + i * 30, chy, 26, 66, 5)
        src(c, "#dbeafe" if i < 3 else "#f1f5f9"); c.fill_preserve()
        src(c, "#93c5fd"); c.set_line_width(1.4); c.stroke()
    text(c, chx + 90, chy + 86, "queued tasks", 10.5, color=MUTED, align="center")
    Co = box(c, 1230, 470, 150, 66, "consumer", ["next worker"], DECODE, title_size=13)

    edge(c, Pr["r"], Sem["l"], LINE)
    edge(c, Sem["r"], (chx - 6, chy + 33), GREEN, label="permit")
    edge(c, (chx + 180, chy + 33), Co["l"], LINE)
    poly(c, [Co["b"], (Co["b"][0], 580), (Sem["b"][0], 580), Sem["b"]],
         RED, dashed=True, label="drop permit when task done", label_idx=1)

    # task struct
    tx, ty = 620, 620
    rrect(c, tx, ty, 760, 205, 12); src(c, WHITE); c.fill_preserve()
    src(c, PANEL_LINE); c.set_line_width(1.5); c.stroke()
    text(c, tx + 18, ty + 26, "SCloudWorkerTask (rides inside InFlightTask { task, _permit })", 12.5, bold=True, color=INK)
    fields = [
        "task_id: Uuid", "for_type: WorkerType", "for_who: SocketAddr",
        "payload: Bytes", "attempts / max_attempts: u8", "created_at: SystemTime",
        "deadline_timeout: Option<SystemTime>", "priority: u8",
        "reply_to: Option<String>  (\"doh\" tag)", "correlation_id: Option<String>",
    ]
    for i, f in enumerate(fields):
        col = i % 2
        row = i // 2
        text(c, tx + 30 + col * 380, ty + 58 + row * 30, "• " + f, 11.5, color=MUTED)

    footer(c, W, H, "05 · workers/mod.rs · manager/mod.rs · task.rs")


# =====================================================================
# 06 - DNS message layout (protocol reference)
# =====================================================================
def d06(c):
    W, H = 1320, 940
    title_block(c, W, "DNS Message Layout — wire format ↔ Rust structs",
                "How DECODER/ENCODER should map bytes to src/dns/packet/*  (from_bytes / to_bytes)")

    # message sections stack
    panel(c, 40, 110, 300, 470, "DNS message sections")
    secs = [
        ("Header", "12 bytes, fixed", DECODE, "Header"),
        ("Question", "QDCOUNT entries", INGRESS, "QuestionSection"),
        ("Answer", "ANCOUNT RRs", RESOLVE, "AnswerSection"),
        ("Authority", "NSCOUNT RRs", CACHE, "AuthoritySection"),
        ("Additional", "ARCOUNT RRs", SIDE, "AdditionalSection"),
    ]
    yy = 150
    for name, sub, cat, struct in secs:
        h = 70 if name == "Header" else 78
        b = box(c, 62, yy, 256, h, name, [sub, "→ " + struct], cat, title_size=15)
        yy += h + 8

    # header bit table
    panel(c, 370, 110, 910, 300, "Header — 12 bytes (6 × 16-bit words)")
    tblx, tbly = 392, 150
    cellw = 54.0  # 16 bits per row
    roww = cellw * 16

    def bitrow(y, cells, label):
        text(c, tblx - 8, y + 20, label, 10, color=MUTED, align="right")
        x = tblx
        for name, bits, col in cells:
            w = cellw * bits
            rrect(c, x, y, w - 2, 30, 4)
            src(c, col); c.fill_preserve(); src(c, "#cbd5e1"); c.set_line_width(1); c.stroke()
            text(c, x + w / 2, y + 15, name, 10.5, bold=True, color=INK, align="center")
            text(c, x + w / 2, y + 27, f"{bits}b", 8, color=MUTED, align="center")
            x += w

    text(c, tblx + roww / 2, 142, "bit  0 → 15", 9.5, color=MUTED, align="center")
    bitrow(150, [("ID", 16, "#e0e7ff")], "0")
    bitrow(186, [("QR", 1, "#fee2e2"), ("Opcode", 4, "#fef3c7"), ("AA", 1, "#fee2e2"),
                 ("TC", 1, "#fee2e2"), ("RD", 1, "#fee2e2"), ("RA", 1, "#fee2e2"),
                 ("Z", 3, "#f1f5f9"), ("RCODE", 4, "#fef3c7")], "2 (flags)")
    bitrow(222, [("QDCOUNT", 16, "#dcfce7")], "4")
    bitrow(258, [("ANCOUNT", 16, "#dcfce7")], "6")
    bitrow(294, [("NSCOUNT", 16, "#dcfce7")], "8")
    bitrow(330, [("ARCOUNT", 16, "#dcfce7")], "10")
    text(c, 392, 384, "flags packed by Header::to_bytes(): QR<<7 | Opcode<<3 | AA<<2 | TC<<1 | RD, then RA<<7 | Z<<4 | RCODE",
         10.5, italic=True, color=MUTED)

    # question format
    panel(c, 370, 430, 440, 210, "Question format → QuestionSection")
    qx = 392
    box(c, qx, 470, 396, 44, "QNAME", ["length-prefixed labels · 0x00 terminator"], INGRESS, title_size=13)
    box(c, qx, 524, 190, 44, "QTYPE", ["A/AAAA/CNAME/MX…"], INGRESS, title_size=12)
    box(c, qx + 206, 524, 190, 44, "QCLASS", ["IN"], INGRESS, title_size=12)
    text(c, qx, 596, "q_name: String · q_type: DNSRecordType · q_class: DNSClass", 10.5, color=MUTED)
    text(c, qx, 616, "labels support compression pointers (0xC0) when parsing.", 10.5, italic=True, color=MUTED)

    # resource record format
    panel(c, 840, 430, 440, 210, "Resource record → Answer/Authority/Additional")
    rx = 862
    for i, (nm, sub) in enumerate([("NAME", "owner"), ("TYPE", "16b"), ("CLASS", "16b")]):
        box(c, rx + i * 132, 470, 124, 40, nm, [sub], RESOLVE, title_size=12)
    box(c, rx, 520, 124, 40, "TTL", ["u32 secs"], RESOLVE, title_size=12)
    box(c, rx + 132, 520, 124, 40, "RDLENGTH", ["u16"], RESOLVE, title_size=11)
    box(c, rx + 264, 520, 124, 40, "RDATA", ["rdlength B"], RESOLVE, title_size=12)
    text(c, rx, 596, "ttl: u32 · rdlength: u16 · rdata: Vec<u8> (A = 4 bytes, …)", 10.5, color=MUTED)

    # flow decode/encode
    panel(c, 40, 660, 1240, 190, "Decode / encode responsibilities")
    box(c, 70, 705, 220, 60, "raw bytes", ["UDP / TCP / DoH body"], CLIENT, title_size=13)
    De = box(c, 340, 705, 220, 60, "DECODER", ["DNSPacket::from_bytes"], DECODE, title_size=13)
    Wk = box(c, 620, 705, 220, 60, "pipeline", ["cache / resolve / build"], ROUTE, title_size=13)
    En = box(c, 900, 705, 220, 60, "ENCODER", ["DNSPacket::to_bytes"], DECODE, title_size=13)
    Se = box(c, 1150, 720, 110, 44, "SENDER", None, EGRESS, title_size=12)
    edge(c, r((70, 705, 220, 60))["r"], De["l"], LINE)
    edge(c, De["r"], Wk["l"], PURPLE)
    edge(c, Wk["r"], En["l"], PURPLE)
    edge(c, En["r"], Se["l"], EGRESS[0])
    text(c, 340, 800, "DECODER should parse the query; ENCODER should set qr=1, fill answers, recompute counts, and serialize.",
         11, italic=True, color=MUTED)

    footer(c, W, H, "06 · src/dns/packet/{header,question,answer,authority,additional}")


# =====================================================================
if __name__ == "__main__":
    print("Rendering diagrams into", OUT)
    render("01_system_context", 1300, 820, d01, OUT)
    render("02_pipeline_intended", 1800, 900, d02, OUT)
    render("03_pipeline_current", 1800, 820, d03, OUT)
    render("04_request_lifecycle", 1560, 940, d04, OUT)
    render("05_worker_lifecycle", 1500, 900, d05, OUT)
    render("06_dns_message", 1320, 940, d06, OUT)
    print("done.")

const std = @import("std");
const mem = std.mem;

const input = @embedFile("input.txt");

fn p1() !void {
    var score: u32 = 0;

    var lines = mem.tokenizeAny(u8, input, "\r\n");

    while (lines.next()) |line| {
        const start = mem.indexOfScalar(u8, line, ':').?;
        const middle = mem.indexOfScalar(u8, line, '|').?;

        var points: u32 = 0;

        var numbers = mem.tokenizeScalar(u8, line[middle + 2 ..], ' ');
        while (numbers.next()) |number| {
            var winning = mem.tokenizeScalar(u8, line[start + 2 .. middle - 1], ' ');

            while (winning.next()) |win| {
                if (mem.eql(u8, number, win)) {
                    if (points == 0) {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                }
            }
        }

        score += points;
    }

    std.debug.print("{d}", .{score});
}

fn p2() !void {
    comptime var num_lines = 1;
    comptime {
        // this is bad
        @setEvalBranchQuota(input.len + 1000);
        num_lines += mem.count(u8, input, "\n");
    }

    var copies = mem.zeroes([num_lines]u32);

    var lines = mem.tokenizeAny(u8, input, "\r\n");
    var i: usize = 0;
    while (lines.next()) |line| : (i += 1) {
        const start = mem.indexOfScalar(u8, line, ':').?;
        const middle = mem.indexOfScalar(u8, line, '|').?;

        var new_copies: u32 = 0;

        var numbers = mem.tokenizeScalar(u8, line[middle + 2 ..], ' ');
        while (numbers.next()) |number| {
            var winning = mem.tokenizeScalar(u8, line[start + 2 .. middle - 1], ' ');

            while (winning.next()) |win| {
                if (mem.eql(u8, number, win)) {
                    new_copies += 1;
                }
            }
        }

        copies[i] += 1;

        for (i + 1..i + new_copies + 1) |j| {
            copies[j] += copies[i];
        }
    }

    var total: u32 = 0;
    for (copies) |n| {
        total += n;
    }

    std.debug.print("{d}", .{total});
}

pub fn main() !void {
    try p2();
}

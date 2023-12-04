const std = @import("std");
const data = @embedFile("input.txt");

fn p1() !void {
    var total: u32 = 0;

    var ls = std.mem.tokenizeScalar(u8, data, '\n');

    while (ls.next()) |line| {
        var first: u32 = undefined;
        var last: u32 = undefined;

        for (line) |c| {
            if (std.ascii.isDigit(c)) {
                first = try std.fmt.charToDigit(c, 10);
                break;
            }
        }

        var i: usize = line.len;
        while (i > 0) {
            i -= 1;
            if (std.ascii.isDigit(line[i])) {
                last = try std.fmt.charToDigit(line[i], 10);
                break;
            }
        }

        total += first * 10 + last;
    }

    std.debug.print("{any}", .{total});
}

pub fn main() !void {
    try p1();
}

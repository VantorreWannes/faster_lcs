const std = @import("std");

pub fn Grid(comptime T: type) type {
    return struct {
        fields: []T,
        width: usize,
        height: usize,
        allocator: std.mem.Allocator,

        pub fn init(allocator: std.mem.Allocator, width: usize, height: usize) !Grid(T) {
            const fields = try allocator.alloc(T, width * height);
            return Grid(T){
                .fields = fields,
                .width = width,
                .height = height,
                .allocator = allocator,
            };
        }

        pub fn deinit(self: Grid(T)) void {
            self.allocator.free(self.fields);
        }

        pub fn get(self: Grid(T), x: usize, y: usize) T {
            return self.fields[x * self.width + y];
        }

        pub fn set(self: Grid(T), x: usize, y: usize, value: T) void {
            self.fields[x * self.width + y] = value;
        }

        pub fn fill(self: Grid(T), value: T) void {
            for (self.fields) |*field| {
                field.* = value;
            }
        }
    };
}

pub const Table = Grid(usize);

pub const Lcs = struct {
    source: []const u8,
    target: []const u8,
    table: Table,

    pub fn init(allocator: std.mem.Allocator, source: []const u8, target: []const u8) !Lcs {
        const table = try Table.init(allocator, source.len + 1, target.len + 1);
        table.fill(0);

        for (0..table.width) |x| {
            for (0..table.height) |y| {
                if (x == 0 or y == 0) {
                    table.set(x, y, 0);
                } else if (source[x - 1] == target[y - 1]) {
                    const new_value = table.get(x - 1, y - 1) + 1;
                    table.set(x, y, new_value);
                } else {
                    const new_value = std.mem.max(usize, &[2]usize{ table.get(x - 1, y), table.get(x, y - 1) });
                    table.set(x, y, new_value);
                }
            }
        }
        return Lcs{
            .source = source,
            .target = target,
            .table = table,
        };
    }

    pub fn deinit(self: Lcs) void {
        self.table.deinit();
    }

    pub fn len(self: Lcs) usize {
        return self.table.get(self.table.width - 1, self.table.height - 1);
    }
};

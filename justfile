# Use `just work day-01 part1` to work on the specific binary for a specific day's problems
work day part:
    cargo watch -w {{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}" -s "just bench {{day}} {{part}}" 
lint day:
    cargo clippy -p {{day}}
test day part:
    cargo nextest run -p {{day}} {{part}}
bench-all:
    cargo bench -q > benchmark-results/bench-all.txt
bench day part:
    cargo bench --bench {{day}}-bench {{part}} >> benchmark-results/{{day}}.bench.txt
dhat day part:
    cargo run --profile dhat --features dhat-heap --package {{day}} --bin {{part}}
# create the directory for a new day's puzzle and fetch the input
create day:
    cargo generate --path ./daily-template --name {{day}}
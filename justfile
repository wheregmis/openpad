run:
    MAKEPAD=lines cargo run -p openpad-app --release

# Run from openpad-app so the full script VM (including openpad-widgets) is loaded
check-script:
    cd openpad-app && cargo makepad check script

serve:
    opencode serve --port 4096
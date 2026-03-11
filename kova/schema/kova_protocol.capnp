# Kova Distributed Computing Swarm — Binary Protocol Schema
# Zero-copy Cap'n Proto definitions for c2-core ↔ kova-node communication.
# All paths are relative to /mnt/hive unless otherwise specified.

@0xdbb9ad1f14bf0b36;

# =============================================================================
# Command — Instructions dispatched from c2-core to worker nodes
# =============================================================================

struct Command {
  # Discriminator for the command variant. Strict deserialization rejects unknown values.
  commandType @0 :CommandType;

  # Base working directory. Default: /mnt/hive/projects
  workingDir @1 :Text = "/mnt/hive/projects";

  # Command-specific payload (union)
  payload :union {
    # Cargo build: compile a Rust project under /mnt/hive/projects/
    cargoBuild @2 :CargoBuildPayload;
    # File sync: rsync/copy between paths on the GlusterFS volume
    fileSync @3 :FileSyncPayload;
    # Reserved for future command types
    unset @4 :Void;
  }
}

enum CommandType {
  cargoBuild @0;
  fileSync @1;
}

struct CargoBuildPayload {
  # Project path relative to workingDir (e.g. "my-crate" or "workspace/crate-a")
  projectPath @0 :Text;
  # Build profile: "debug" or "release"
  profile @1 :Text = "release";
  # Additional cargo build arguments (e.g. --features, --target)
  extraArgs @2 :List(Text);
}

struct FileSyncPayload {
  # Source path (absolute or relative to /mnt/hive)
  source @0 :Text;
  # Destination path
  destination @1 :Text;
}

# =============================================================================
# Telemetry — Structured feedback streamed from worker nodes to c2-core
# =============================================================================

struct Telemetry {
  # Node identifier (hostname or configured ID)
  nodeId @0 :Text;

  # Unix timestamp (nanoseconds since epoch)
  timestampNs @1 :Int64;

  # CPU thermal reading in Celsius. -1.0 if unavailable.
  cpuThermalCelsius @2 :Float32 = -1.0;

  # Memory utilization: 0.0 (idle) to 1.0 (fully utilized)
  memoryUtilization @3 :Float32;

  # Optional: command execution result when reporting completion
  result @4 :Result;
}

struct Result {
  success @0 :Bool;
  # Exit code for process-based commands (e.g. cargo build)
  exitCode @1 :Int32 = -1;
  # Stderr / error message on failure
  errorMessage @2 :Text;
}

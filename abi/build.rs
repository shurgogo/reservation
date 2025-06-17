use std::process::Command;

use tonic_build::Builder;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_sqlx_type(&["reservation.ReservationStatus"])
        .with_derive_builder(&["reservation.ReservationQuery"])
        .with_derive_builder_into(
            "reservation.ReservationQuery",
            &[
                "resource_id",
                "user_id",
                "status",
                "page",
                "desc",
                "page_size",
            ],
        )
        .with_derive_builder_option("reservation.ReservationQuery", &["start", "end"])
        .compile_protos(&["protos/reservation.proto"], &["proto"])
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=protos/reservation.proto");
}
trait BuilderAttributes {
    fn with_sqlx_type(self, paths: &[&str]) -> Self;
    fn with_derive_builder(self, paths: &[&str]) -> Self;
    fn with_derive_builder_into(self, path: &str, fields: &[&str]) -> Self;
    fn with_derive_builder_option(self, path: &str, fields: &[&str]) -> Self;
}

impl BuilderAttributes for Builder {
    fn with_sqlx_type(self, paths: &[&str]) -> Self {
        paths.iter().fold(self, |builder, path| {
            builder.type_attribute(path, "#[derive(sqlx::Type)]")
        })
    }

    fn with_derive_builder(self, paths: &[&str]) -> Self {
        paths.iter().fold(self, |builder, path| {
            builder.type_attribute(path, "#[derive(derive_builder::Builder)]")
        })
    }

    fn with_derive_builder_into(self, path: &str, fields: &[&str]) -> Self {
        fields.iter().fold(self, |builder, field| {
            builder.field_attribute(
                format!("{path}.{field}"),
                "#[builder(setter(into), default)]",
            )
        })
    }

    fn with_derive_builder_option(self, path: &str, fields: &[&str]) -> Self {
        fields.iter().fold(self, |builder, field| {
            builder.field_attribute(
                format!("{path}.{field}"),
                "#[builder(setter(into, strip_option))]",
            )
        })
    }
}

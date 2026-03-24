//! [Volumetric flux](https://en.wikipedia.org/wiki/Volumetric_flux) (base unit cubic meter per square meter second, m⁻³ · m⁻² · s⁻¹).

quantity! {
    /// Volumetric flux (base unit cubic meter per second square meter, m⁻³ · m⁻² · s⁻¹).
    quantity: VolumetricFlux; "volumetric flux";
    /// Dimension of volumetric flux, LT⁻¹ (base unit meter per second, m · s⁻¹)
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::FluxKind;
    units {
        @cubic_meter_per_square_meter_second: prefix!(none); "m³/(m² · s)", "cubic meter per square meter second",
            "cubic meter per square meter second";
        @cubic_meter_per_square_meter_hour: 2.777_777_777_777_778_E-4; "m³/(m² · h)", "cubic meter per square meter hour",
            "cubic meter per square meter hour";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::area as area;
        use crate::si::quantities::*;
        use crate::si::time as time;
        use crate::si::volume as vol;
        use crate::si::volumetric_flux as vflux;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: VolumetricFlux<V> = (Volume::new::<vol::cubic_meter>(V::one())
                / (Area::new::<area::square_meter>(V::one()) * Time::new::<time::second>(V::one()))).into();
        }

        #[test]
        fn check_heat_capacity_volume_units() {
            test::<vol::cubic_meter, area::square_meter, time::second, vflux::cubic_meter_per_square_meter_second>();
            test::<vol::cubic_meter, area::square_meter, time::hour,   vflux::cubic_meter_per_square_meter_hour>();

            fn test<
                VOL: vol::Conversion<V>,
                AREA: area::Conversion<V>,
                TIME: time::Conversion<V>,
                VFLUX: vflux::Conversion<V>>()
            {
                Test::assert_approx_eq(&VolumetricFlux::new::<VFLUX>(V::one()),
                    &((Volume::new::<VOL>(V::one())
                        / (Area::new::<AREA>(V::one()) * Time::new::<TIME>(V::one()))).into())
                );
            }
        }
    }
}

//! Port of utils/emissions/HelpersHarmonoise.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Helper methods for Harmonoise-based noise emission computation
/// C++ class `HelpersHarmonoise` (utils/emissions/HelpersHarmonoise.h)
pub struct HelpersHarmonoise {
}

// C++ static members of HelpersHarmonoise (port as module-level statics, see AGENTS.md):
//   static double[27] HelpersHarmonoise::myR_A_C1_Parameter;
//   static double[27] HelpersHarmonoise::myR_B_C1_Parameter;
//   static double[27] HelpersHarmonoise::myR_A_C3_Parameter;
//   static double[27] HelpersHarmonoise::myR_B_C3_Parameter;
//   static double[27] HelpersHarmonoise::myT_A_C1_Parameter;
//   static double[27] HelpersHarmonoise::myT_B_C1_Parameter;
//   static double[27] HelpersHarmonoise::myT_A_C3_Parameter;
//   static double[27] HelpersHarmonoise::myT_B_C3_Parameter;
//   static const double[27] HelpersHarmonoise::myAOctaveBandCorrection;
//   static const double[27] HelpersHarmonoise::mySurfaceCorrection;

impl HelpersHarmonoise {
    /// C++: `double HelpersHarmonoise::computeNoise(SUMOEmissionClass, double, double)` [static]
    /// Returns the noise produced by the a vehicle of the given type at the given speed
    // double
    // HelpersHarmonoise::computeNoise(SUMOEmissionClass c, double v, double a) {
    //     double* alphaT, *betaT, *alphaR, *betaR;
    //     double ac = 0;
    //     if (PollutantsInterface::isHeavy(c)) {
    //         alphaT = myT_A_C3_Parameter;
    //         betaT = myT_B_C3_Parameter;
    //         alphaR = myR_A_C3_Parameter;
    //         betaR = myR_B_C3_Parameter;
    //         ac = 5.6;
    //     } else if (!PollutantsInterface::isSilent(c)) {
    //         alphaT = myT_A_C1_Parameter;
    //         betaT = myT_B_C1_Parameter;
    //         alphaR = myR_A_C1_Parameter;
    //         betaR = myR_B_C1_Parameter;
    //         ac = 4.4;
    //     } else {
    //         return 0;
    //     }
    //     //
    //     double L_low = 0;
    //     double L_high = 0;
    //     v = v * 3.6;
    //     double s = -30.;//
    //     for (int i = 0; i < 27; ++i) {
    //         double crc_low = alphaR[i] + betaR[i] * log10(v / 70.) + 10.*log10(.8); // + mySurfaceCorrection[i];
    //         double ctc_low = alphaT[i] + betaT[i] * ((v - 70.) / 70.) + a * ac + 10.*log10(.2);
    //         double Li_low = 10. * log10(pow(10., (crc_low / 10.)) + pow(10., (ctc_low / 10.)));
    //         Li_low += s;
    //         double crc_high = alphaR[i] + betaR[i] * log10(v / 70.) + 10.*log10(.2); // + mySurfaceCorrection[i];
    //         double ctc_high = alphaT[i] + betaT[i] * ((v - 70.) / 70.) + a * ac + 10.*log10(.8);
    //         double Li_high = 10. * log10(pow(10., (crc_high / 10.)) + pow(10., (ctc_high / 10.)));
    //         Li_high += s;
    //         L_low += pow(10., (Li_low + myAOctaveBandCorrection[i]) / 10.);
    //         L_high += pow(10., (Li_high + myAOctaveBandCorrection[i]) / 10.);
    //     }
    //     L_low = (10. * log10(L_low));
    //     L_high = (10. * log10(L_high));
    //     double v1 = (double)(10. * log10(pow(10., L_low / 10.) + pow(10., L_high / 10.)));
    //     return v1;
    // }
    #[allow(unused_variables)]
    pub fn compute_noise(c: SUMOEmissionClass, v: f64, a: f64) -> f64 {
        todo!("port HelpersHarmonoise::computeNoise")
    }

    /// C++: `double HelpersHarmonoise::sum(double)` [static]
    /// Computes the resulting noise
    // inline static double sum(double val) {
    //         return double(10. * log10(val));
    //     }
    #[allow(unused_variables)]
    pub fn sum(val: f64) -> f64 {
        todo!("port HelpersHarmonoise::sum")
    }

}


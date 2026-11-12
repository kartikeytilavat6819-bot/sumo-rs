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
const MY_R_A_C1_PARAMETER: [f64; 27] = [69.9, 69.9, 69.9, 74.9, 74.9, 74.9, 77.3, 77.5, 78.1, 78.3, 78.9, 77.8, 78.5, 81.9, 84.1, 86.5, 88.6, 88.2, 87.6, 85.8, 82.8, 80.2, 77.6, 75.0, 72.8, 70.4, 67.9];
const MY_R_B_C1_PARAMETER: [f64; 27] = [33.0, 33.0, 33.0, 15.2, 15.2, 15.2, 41.0, 41.2, 42.3, 41.8, 38.6, 35.5, 31.7, 21.5, 21.2, 23.5, 29.1, 33.5, 34.1, 35.1, 36.4, 37.4, 38.9, 39.7, 39.7, 39.7, 39.7];
const MY_R_A_C3_PARAMETER: [f64; 27] = [80.5, 80.5, 80.5, 82.5, 83.5, 83.5, 86.5, 88.3, 88.7, 88.3, 91.4, 92.2, 96.0, 98.1, 97.8, 98.4, 97.2, 94.6, 95.9, 90.5, 87.1, 85.1, 83.2, 81.3, 81.3, 81.3, 81.3];
const MY_R_B_C3_PARAMETER: [f64; 27] = [33.0, 33.0, 33.0, 30.0, 30.0, 30.0, 41.0, 41.2, 42.3, 41.8, 38.6, 35.5, 31.7, 21.5, 21.2, 23.5, 29.1, 33.5, 34.1, 35.1, 36.4, 37.4, 38.9, 39.7, 39.7, 39.7, 39.7];
const MY_T_A_C1_PARAMETER: [f64; 27] = [90.0, 92.0, 89.0, 91.0, 92.4, 94.8, 90.8, 86.8, 86.2, 84.5, 84.5, 84.8, 83.5, 81.8, 81.4, 79.0, 79.2, 81.4, 85.5, 85.8, 85.2, 82.9, 81.0, 78.2, 77.2, 75.2, 74.2];
const MY_T_B_C1_PARAMETER: [f64; 27] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4, 9.4];
const MY_T_A_C3_PARAMETER: [f64; 27] = [97.7, 97.3, 98.2, 103.3, 109.5, 104.3, 99.8, 100.2, 98.9, 99.5, 100.7, 101.2, 100.6, 100.2, 97.4, 97.1, 97.8, 97.3, 95.8, 94.9, 92.7, 90.6, 89.9, 87.9, 85.9, 83.8, 82.2];
const MY_T_B_C3_PARAMETER: [f64; 27] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7, 11.7];
const MY_AOCTAVE_BAND_CORRECTION: [f64; 27] = [-44.7, -39.4, -34.6, -30.2, -26.2, -22.5, -19.1, -16.1, -13.4, -10.9, -8.6, -6.6, -4.8, -3.2, -1.9, -0.8, 0.0, 0.6, 1.0, 1.2, 1.3, 1.2, 1.0, 0.5, -0.1, -1.1, -2.5];
const MY_SURFACE_CORRECTION: [f64; 27] = [0.7, 0.2, 3.6, -1.0, -1.8, -0.1, -0.9, -0.7, -1.1, -0.5, -1.5, -2.4, -3.0, -4.6, -5.8, -6.5, -7.9, -7.8, -7.2, -6.3, -5.6, -5.5, -4.8, -4.3, 0., 0., 0.];

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
        let (alphaT, betaT, alphaR, betaR, ac) = if PollutantsInterface::is_heavy(c) {
            (&MY_T_A_C3_PARAMETER, &MY_T_B_C3_PARAMETER, &MY_R_A_C3_PARAMETER, &MY_R_B_C3_PARAMETER, 5.6)
        } else if !PollutantsInterface::is_silent(c) {
            (&MY_T_A_C1_PARAMETER, &MY_T_B_C1_PARAMETER, &MY_R_A_C1_PARAMETER, &MY_R_B_C1_PARAMETER, 4.4)
        } else {
            return 0.;
        };
        //
        let mut L_low = 0.;
        let mut L_high = 0.;
        let v = v * 3.6;
        let s = -30.; //
        for i in 0..27 {
            let crc_low = alphaR[i] + betaR[i] * (v / 70.).log10() + 10. * (0.8f64).log10(); // + mySurfaceCorrection[i];
            let ctc_low = alphaT[i] + betaT[i] * ((v - 70.) / 70.) + a * ac + 10. * (0.2f64).log10();
            let mut Li_low = 10. * (10f64.powf(crc_low / 10.) + 10f64.powf(ctc_low / 10.)).log10();
            Li_low += s;
            let crc_high = alphaR[i] + betaR[i] * (v / 70.).log10() + 10. * (0.2f64).log10(); // + mySurfaceCorrection[i];
            let ctc_high = alphaT[i] + betaT[i] * ((v - 70.) / 70.) + a * ac + 10. * (0.8f64).log10();
            let mut Li_high = 10. * (10f64.powf(crc_high / 10.) + 10f64.powf(ctc_high / 10.)).log10();
            Li_high += s;
            L_low += 10f64.powf((Li_low + MY_AOCTAVE_BAND_CORRECTION[i]) / 10.);
            L_high += 10f64.powf((Li_high + MY_AOCTAVE_BAND_CORRECTION[i]) / 10.);
        }
        L_low = 10. * L_low.log10();
        L_high = 10. * L_high.log10();
        let v1 = 10. * (10f64.powf(L_low / 10.) + 10f64.powf(L_high / 10.)).log10();
        v1
    }

    /// C++: `double HelpersHarmonoise::sum(double)` [static]
    /// Computes the resulting noise
    // inline static double sum(double val) {
    //         return double(10. * log10(val));
    //     }
    #[allow(unused_variables)]
    pub fn sum(val: f64) -> f64 {
        10. * val.log10()
    }

}


//! Port of utils/emissions/CharacteristicMap.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The purpose of this class is to store a characteristic map (German: Kennfeld) of arbitrary dimensions and to provide functions in order to evaluate/interpolate points in it.
/// C++ class `CharacteristicMap` (utils/emissions/CharacteristicMap.h)
pub struct CharacteristicMap {
    /// Dimension of the map's domain
    pub domainDim: i32,
    /// Image dimension of the map
    pub imageDim: i32,
    /// Vector containing the values along each domain axis in ascending order
    pub axes: Vec<Vec<f64>>,
    /// Flattened map entries
    pub flattenedMap: Vec<f64>,
    /// Stride for each domain dimension in the flattened map
    pub strides: Vec<i32>,
}

impl CharacteristicMap {
    /// C++: `void CharacteristicMap::determineStrides()` [private]
    /// Determine the stride for each map dimension in the flattened map.
    // void
    // CharacteristicMap::determineStrides() {
    //     strides.clear();
    //     strides.reserve(domainDim);
    //     strides.push_back(imageDim);
    //     for (int i = 1; i < domainDim; i++) {
    //         strides.push_back((int)axes[i - 1].size() * strides[i - 1]);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn determine_strides(&mut self) {
        todo!("port CharacteristicMap::determineStrides")
    }

    /// C++: `int CharacteristicMap::calcFlatIdx(const std::vector<int> &) const` [const, private]
    /// Compute the index of a map entry in the flattened map.
    // int
    // CharacteristicMap::calcFlatIdx(const std::vector<int>& ref_idxs) const {
    //     if (static_cast<int>(ref_idxs.size()) != domainDim) {
    //         throw std::runtime_error("The number of indices differs from the map's"
    //                                  " domain dimension.");
    //     }
    //
    //     int flatIdx = 0;
    //     for (int i = 0; i < domainDim; i++) {
    //         if (ref_idxs[i] < 0) {
    //             throw std::runtime_error("The argument indices aren't non-negative.");
    //         }
    //         flatIdx += ref_idxs[i] * strides[i];
    //     }
    //     return flatIdx;
    // }
    #[allow(unused_variables)]
    pub fn calc_flat_idx(&self, ref_idxs: &Vec<i32>) -> i32 {
        todo!("port CharacteristicMap::calcFlatIdx")
    }

    /// C++: `int CharacteristicMap::findNearestNeighborIdxs(const std::vector<double> &, std::vector<int> &, double) const` [const, private]
    /// Determine the indices of the nearest neighbor of a point in the map.
    // int
    // CharacteristicMap::findNearestNeighborIdxs(const std::vector<double>& ref_p,
    //         std::vector<int>& ref_idxs, double eps) const {
    //     if (static_cast<int>(ref_p.size()) != domainDim) {
    //         throw std::runtime_error("The argument point's size doesn't match the"
    //                                  " domain dimension.");
    //     }
    //
    //     ref_idxs = std::vector<int>(domainDim, -1);
    //     for (int i = 0; i < domainDim; i++) {
    //         if (axes[i][0] - eps <= ref_p[i] && ref_p[i] < axes[i][0]) {
    //             ref_idxs[i] = 0;
    //         } else if (axes[i][axes[i].size() - 1] <= ref_p[i]
    //                    && ref_p[i] < axes[i][axes[i].size() - 1] + eps) {
    //             ref_idxs[i] = (int)axes[i].size() - 1;
    //         } else {
    //             for (int j = 0; j < static_cast<int>(axes[i].size()) - 1; j++) {
    //                 if (axes[i][j] <= ref_p[i] && ref_p[i] < axes[i][j + 1]) {
    //                     // Pick the index that is closest to the point
    //                     if (ref_p[i] - axes[i][j] <= axes[i][j + 1] - ref_p[i]) {
    //                         ref_idxs[i] = j;
    //                         break;
    //                     } else {
    //                         ref_idxs[i] = j + 1;
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //
    //         // The point lies outside of the valid range
    //         if (ref_idxs[i] == -1) {
    //             return -1;
    //         }
    //     }
    //
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn find_nearest_neighbor_idxs(&self, ref_p: &Vec<f64>, ref_idxs: &mut Vec<i32>, eps: f64) -> i32 {
        todo!("port CharacteristicMap::findNearestNeighborIdxs")
    }

    /// C++: `std::vector<double> CharacteristicMap::at(const std::vector<int> &) const` [const, private]
    /// Access a map entry using its indices.
    // std::vector<double>
    // CharacteristicMap::at(const std::vector<int>& ref_idxs) const {
    //     if (static_cast<int>(ref_idxs.size()) != domainDim) {
    //         throw std::runtime_error("The number of indices differs from the map's"
    //                                  " domain dimension.");
    //     }
    //
    //     int flatIdx = calcFlatIdx(ref_idxs);
    //     return std::vector<double>(flattenedMap.begin() + flatIdx,
    //                                flattenedMap.begin() + flatIdx + imageDim);
    // }
    #[allow(unused_variables)]
    pub fn at(&self, ref_idxs: &Vec<i32>) -> Vec<f64> {
        todo!("port CharacteristicMap::at")
    }

    /// C++: `void CharacteristicMap::CharacteristicMap(int, int, const std::vector<std::vector<double>> &, const std::vector<double> &)`
    /// Constructor
    // CharacteristicMap::CharacteristicMap(int domainDim, int imageDim,
    //                                      const std::vector<std::vector<double>>& ref_axes,
    //                                      const std::vector<double>& ref_flattenedMap)
    //     : domainDim(domainDim),
    //       imageDim(imageDim),
    //       axes(ref_axes),
    //       flattenedMap(ref_flattenedMap) {
    //     // Check whether the dimensions are consistent
    //     if (static_cast<int>(axes.size()) != domainDim) {
    //         throw std::runtime_error("The number of axes doesn't match the specified"
    //                                  " domain dimension.");
    //     }
    //     int expectedEntryCnt = imageDim;
    //     for (auto& ref_axis : axes) {
    //         expectedEntryCnt *= (int)ref_axis.size();
    //     }
    //     if (static_cast<int>(flattenedMap.size()) != expectedEntryCnt) {
    //         throw std::runtime_error("The number of map entries isn't equal to the"
    //                                  " product of the axes' dimensions times the image dimension.");
    //     }
    //
    //     determineStrides();
    // }
    #[allow(unused_variables)]
    pub fn new(domainDim: i32, imageDim: i32, ref_axes: &Vec<Vec<f64>>, ref_flattenedMap: &Vec<f64>) -> Self {
        todo!("port CharacteristicMap::CharacteristicMap")
    }

    /// C++: `void CharacteristicMap::CharacteristicMap(const std::string &)`
    /// Constructor
    // CharacteristicMap::CharacteristicMap(const std::string& ref_mapString) {
    //     // Split the map string into its three main parts
    //     const std::vector<std::string> tokens = StringTokenizer(ref_mapString, "|").getVector();
    //     if (tokens.size() != 3) {
    //         throw std::runtime_error("The map string isn't made up of the 3 parts"
    //                                  " dimensions, axes, and flattened entries.");
    //     }
    //
    //     // Extract the domain and image dimensions
    //     const std::vector<std::string> dimensionTokens = StringTokenizer(tokens[0], ",").getVector();
    //     if (dimensionTokens.size() != 2) {
    //         throw std::runtime_error("The domain and image dimensions aren't specified"
    //                                  " correctly.");
    //     }
    //     domainDim = std::stoi(dimensionTokens[0]);
    //     imageDim = std::stoi(dimensionTokens[1]);
    //
    //     // Create the map axes
    //     const std::vector<std::string> axisTokens = StringTokenizer(tokens[1], ";").getVector();
    //     if (static_cast<int>(axisTokens.size()) != domainDim) {
    //         throw std::runtime_error("The number of axes doesn't match the specified"
    //                                  " domain dimension.");
    //     }
    //     for (auto& ref_axisToken : axisTokens) {
    //         std::vector<std::string> axisEntryTokens = StringTokenizer(ref_axisToken, ",").getVector();
    //         std::vector<double> axisEntries;
    //         for (auto& ref_axisEntryToken : axisEntryTokens) {
    //             axisEntries.push_back(std::stod(ref_axisEntryToken));
    //         }
    //         axes.push_back(axisEntries);
    //     }
    //
    //     // Create the flattened map
    //     const std::vector<std::string> flattenedMapTokens = StringTokenizer(tokens[2], ",").getVector();
    //     int expectedEntryCnt = imageDim;
    //     for (auto& ref_axis : axes) {
    //         expectedEntryCnt *= (int)ref_axis.size();
    //     }
    //     if (static_cast<int>(flattenedMapTokens.size()) != expectedEntryCnt) {
    //         throw std::runtime_error("The number of map entries isn't equal to the"
    //                                  " product of the axes' dimensions times the image dimension.");
    //     }
    //     flattenedMap.reserve(expectedEntryCnt);
    //     for (auto& ref_flattenedMapToken : flattenedMapTokens) {
    //         flattenedMap.push_back(std::stod(ref_flattenedMapToken));
    //     }
    //
    //     determineStrides();
    // }
    #[allow(unused_variables)]
    pub fn new_2(ref_mapString: &str) -> Self {
        todo!("port CharacteristicMap::CharacteristicMap")
    }

    /// C++: `std::string CharacteristicMap::toString() const` [const]
    /// Encode the map as a string.
    // std::string
    // CharacteristicMap::toString() const {
    //     // Write the domain and image dimensions
    //     std::string mapString = std::to_string(domainDim) + ","
    //                             + std::to_string(imageDim) + "|";
    //
    //     // Add the axes
    //     for (int i = 0; i < static_cast<int>(axes.size()); i++) {
    //         for (int j = 0; j < static_cast<int>(axes[i].size()); j++) {
    //             mapString += std::to_string(axes[i][j])
    //                          + (j == static_cast<int>(axes[i].size()) - 1 ? "" : ",");
    //         }
    //         mapString += (i == static_cast<int>(axes.size()) - 1 ? "|" : ";");
    //     }
    //
    //     // Append the flattened map entries
    //     for (int i = 0; i < static_cast<int>(flattenedMap.size()); i++) {
    //         mapString += std::to_string(flattenedMap[i])
    //                      + (i == static_cast<int>(flattenedMap.size()) - 1 ? "" : ",");
    //     }
    //
    //     return mapString;
    // }
    #[allow(unused_variables)]
    pub fn to_string(&self) -> String {
        todo!("port CharacteristicMap::toString")
    }

    /// C++: `int CharacteristicMap::getDomainDim() const` [const]
    /// Get the dimension of the map's domain.
    // int
    // CharacteristicMap::getDomainDim() const {
    //     return domainDim;
    // }
    #[allow(unused_variables)]
    pub fn get_domain_dim(&self) -> i32 {
        todo!("port CharacteristicMap::getDomainDim")
    }

    /// C++: `int CharacteristicMap::getImageDim() const` [const]
    /// Get the image dimension of the map.
    // int
    // CharacteristicMap::getImageDim() const {
    //     return imageDim;
    // }
    #[allow(unused_variables)]
    pub fn get_image_dim(&self) -> i32 {
        todo!("port CharacteristicMap::getImageDim")
    }

    /// C++: `std::vector<double> CharacteristicMap::eval(const std::vector<double> &, double) const` [const]
    /// Evaluate a point in the map using linear interpolation.
    // std::vector<double>
    // CharacteristicMap::eval(const std::vector<double>& ref_p, double eps) const {
    //     if (static_cast<int>(ref_p.size()) != domainDim) {
    //         throw std::runtime_error("The argument's size doesn't match the domain"
    //                                  " dimension.");
    //     }
    //
    //     // Find the nearest neighbor and its image values
    //     std::vector<int> nnIdxs;
    //     if (findNearestNeighborIdxs(ref_p, nnIdxs, eps)) {
    //         return std::vector<double>(imageDim, std::stod("nan"));
    //     }
    //     // Image values of the nearest neighbor
    //     const std::vector<double> y_nn = at(nnIdxs);
    //     // The result is based on the image values of the nearest neighbor
    //     std::vector<double> y = y_nn;
    //
    //     // Interpolate
    //     for (int i = 0; i < domainDim; i++) {
    //         // Depending on the configuration of the points, different neighbors will be
    //         // used for interpolation
    //         const double s = ref_p[i] - axes[i][nnIdxs[i]];
    //         if (std::abs(s) <= eps) {
    //             continue;
    //         }
    //         bool b_constellation1 = s < 0 && nnIdxs[i] > 0;
    //         bool b_constellation2 = s >= 0
    //                                 && nnIdxs[i] == static_cast<int>(axes[i].size()) - 1
    //                                 && nnIdxs[i] > 0;
    //         bool b_constellation3 = s < 0 && nnIdxs[i] == 0
    //                                 && nnIdxs[i] < static_cast<int>(axes[i].size()) - 1;
    //         bool b_constellation4 = s >= 0
    //                                 && nnIdxs[i] < static_cast<int>(axes[i].size()) - 1;
    //
    //         double dx = 1;
    //         // Axis neighbor indices (i.e. the indices of the second support point)
    //         std::vector<int> anIdxs = nnIdxs;
    //         if (b_constellation1 || b_constellation2) {
    //             anIdxs[i] -= 1;
    //             dx = axes[i][nnIdxs[i]] - axes[i][anIdxs[i]];
    //         } else if (b_constellation3 || b_constellation4) {
    //             anIdxs[i] += 1;
    //             dx = axes[i][anIdxs[i]] - axes[i][nnIdxs[i]];
    //         } else {
    //             continue;
    //         }
    //         // Image values of the axis neighbor
    //         const std::vector<double> y_an = at(anIdxs);
    //
    //         for (int j = 0; j < imageDim; j++) {
    //             double dy = 0;
    //             if (b_constellation1 || b_constellation2) {
    //                 dy = y_nn[j] - y_an[j];
    //             } else {
    //                 dy = y_an[j] - y_nn[j];
    //             }
    //
    //             // Update
    //             y[j] += s * dy / dx;
    //         }
    //     }
    //
    //     return y;
    // }
    #[allow(unused_variables)]
    pub fn eval(&self, ref_p: &Vec<f64>, eps: f64) -> Vec<f64> {
        todo!("port CharacteristicMap::eval")
    }

}


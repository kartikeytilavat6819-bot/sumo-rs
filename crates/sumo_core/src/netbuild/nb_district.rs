//! Port of netbuild/NBDistrict.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<double> NBDistrict::WeightsCont`
pub type NBDistrict_WeightsCont = Vec<f64>;

/// A class representing a single district
/// C++ class `NBDistrict` (netbuild/NBDistrict.h)
#[repr(C)]
pub struct NBDistrict {
    /// C++ base class `Named`
    pub base: Named,
    /// The sources (connection from district to network)
    pub mySources: EdgeVector,
    /// The weights of the sources
    pub mySourceWeights: NBDistrict_WeightsCont,
    /// The sinks (connection from network to district)
    pub mySinks: EdgeVector,
    /// The weights of the sinks
    pub mySinkWeights: NBDistrict_WeightsCont,
    /// The position of the district
    pub myPosition: Position,
    /// The shape of the dsitrict
    pub myShape: PositionVector,
}

impl std::ops::Deref for NBDistrict {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for NBDistrict {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl NBDistrict {
    /// C++: `void NBDistrict::NBDistrict(const std::string &, const Position &)`
    /// Constructor with id, and position
    // NBDistrict::NBDistrict(const std::string& id, const Position& pos)
    //     : Named(StringUtils::convertUmlaute(id)),
    //       myPosition(pos) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, pos: &Position) -> Self {
        Self {
            base: Named::new(&utils::common::string_utils::convert_umlaute(id)),
            mySources: Vec::new(),
            mySourceWeights: Vec::new(),
            mySinks: Vec::new(),
            mySinkWeights: Vec::new(),
            myPosition: *pos,
            myShape: PositionVector::default(),
        }
    }

    /// C++: `void NBDistrict::NBDistrict(const std::string &)`
    /// Constructor without position
    // NBDistrict::NBDistrict(const std::string& id)
    //     : Named(id), myPosition(0, 0) {}
    #[allow(unused_variables)]
    pub fn new_2(id: &str) -> Self {
        Self {
            base: Named::new(id),
            mySources: Vec::new(),
            mySourceWeights: Vec::new(),
            mySinks: Vec::new(),
            mySinkWeights: Vec::new(),
            myPosition: Position::new(0.0, 0.0),
            myShape: PositionVector::default(),
        }
    }

    /// C++: `void NBDistrict::~NBDistrict() noexcept` [virtual]
    /// Destructor
    // NBDistrict::~NBDistrict() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has no body; the owned vectors are released automatically
    }

    /// C++: `bool NBDistrict::addSource(NBEdge *const, double)`
    /// Adds a source
    // bool
    // NBDistrict::addSource(NBEdge* const source, double weight) {
    //     EdgeVector::iterator i = std::find(mySources.begin(), mySources.end(), source);
    //     if (i != mySources.end()) {
    //         return false;
    //     }
    //     mySources.push_back(source);
    //     mySourceWeights.push_back(weight);
    //     assert(source->getID() != "");
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_source(&mut self, source: Ptr<NBEdge>, weight: f64) -> bool {
        if self.mySources.iter().any(|x| *x == source) {
            return false;
        }
        self.mySources.push(source);
        self.mySourceWeights.push(weight);
        debug_assert!(source.get_id() != "");
        true
    }

    /// C++: `bool NBDistrict::addSink(NBEdge *const, double)`
    /// Adds a sink
    // bool
    // NBDistrict::addSink(NBEdge* const sink, double weight) {
    //     EdgeVector::iterator i = std::find(mySinks.begin(), mySinks.end(), sink);
    //     if (i != mySinks.end()) {
    //         return false;
    //     }
    //     mySinks.push_back(sink);
    //     mySinkWeights.push_back(weight);
    //     assert(sink->getID() != "");
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_sink(&mut self, sink: Ptr<NBEdge>, weight: f64) -> bool {
        if self.mySinks.iter().any(|x| *x == sink) {
            return false;
        }
        self.mySinks.push(sink);
        self.mySinkWeights.push(weight);
        debug_assert!(sink.get_id() != "");
        true
    }

    /// C++: `const Position & NBDistrict::getPosition() const` [const]
    /// Returns the position of this district's center
    // const Position& getPosition() const {
    //         return myPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> &Position {
        &self.myPosition
    }

    /// C++: `void NBDistrict::setCenter(const Position &)`
    /// Sets the center coordinates
    // void
    // NBDistrict::setCenter(const Position& pos) {
    //     myPosition = pos;
    // }
    #[allow(unused_variables)]
    pub fn set_center(&mut self, pos: &Position) {
        self.myPosition = *pos;
    }

    /// C++: `void NBDistrict::replaceIncoming(const EdgeVector &, NBEdge *const)`
    /// Replaces incoming edges from the vector (sinks) by the given edge
    // void
    // NBDistrict::replaceIncoming(const EdgeVector& which, NBEdge* const by) {
    //     // temporary structures
    //     EdgeVector newList;
    //     WeightsCont newWeights;
    //     double joinedVal = 0;
    //     // go through the list of sinks
    //     EdgeVector::iterator i = mySinks.begin();
    //     WeightsCont::iterator j = mySinkWeights.begin();
    //     for (; i != mySinks.end(); i++, j++) {
    //         NBEdge* tmp = (*i);
    //         double val = (*j);
    //         if (find(which.begin(), which.end(), tmp) == which.end()) {
    //             // if the current edge shall not be replaced, add to the
    //             //  temporary list
    //             newList.push_back(tmp);
    //             newWeights.push_back(val);
    //         } else {
    //             // otherwise, skip it and add its weight to the one to be inserted
    //             //  instead
    //             joinedVal += val;
    //         }
    //     }
    //     // add the one to be inserted instead
    //     newList.push_back(by);
    //     newWeights.push_back(joinedVal);
    //     // assign to values
    //     mySinks = newList;
    //     mySinkWeights = newWeights;
    // }
    #[allow(unused_variables)]
    pub fn replace_incoming(&mut self, which: &EdgeVector, by: Ptr<NBEdge>) {
        // temporary structures
        let mut newList: EdgeVector = Vec::new();
        let mut newWeights: NBDistrict_WeightsCont = Vec::new();
        let mut joinedVal = 0.0;
        // go through the list of sinks
        for (tmp, val) in self.mySinks.iter().zip(self.mySinkWeights.iter()) {
            if !which.contains(tmp) {
                // if the current edge shall not be replaced, add to the temporary list
                newList.push(*tmp);
                newWeights.push(*val);
            } else {
                // otherwise, skip it and add its weight to the one to be inserted instead
                joinedVal += *val;
            }
        }
        // add the one to be inserted instead
        newList.push(by);
        newWeights.push(joinedVal);
        // assign to values
        self.mySinks = newList;
        self.mySinkWeights = newWeights;
    }

    /// C++: `void NBDistrict::replaceOutgoing(const EdgeVector &, NBEdge *const)`
    /// Replaces outgoing edges from the vector (source) by the given edge
    // void
    // NBDistrict::replaceOutgoing(const EdgeVector& which, NBEdge* const by) {
    //     // temporary structures
    //     EdgeVector newList;
    //     WeightsCont newWeights;
    //     double joinedVal = 0;
    //     // go through the list of sinks
    //     EdgeVector::iterator i = mySources.begin();
    //     WeightsCont::iterator j = mySourceWeights.begin();
    //     for (; i != mySources.end(); i++, j++) {
    //         NBEdge* tmp = (*i);
    //         double val = (*j);
    //         if (find(which.begin(), which.end(), tmp) == which.end()) {
    //             // if the current edge shall not be replaced, add to the
    //             //  temporary list
    //             newList.push_back(tmp);
    //             newWeights.push_back(val);
    //         } else {
    //             // otherwise, skip it and add its weight to the one to be inserted
    //             //  instead
    //             joinedVal += val;
    //         }
    //     }
    //     // add the one to be inserted instead
    //     newList.push_back(by);
    //     newWeights.push_back(joinedVal);
    //     // assign to values
    //     mySources = newList;
    //     mySourceWeights = newWeights;
    // }
    #[allow(unused_variables)]
    pub fn replace_outgoing(&mut self, which: &EdgeVector, by: Ptr<NBEdge>) {
        // temporary structures
        let mut newList: EdgeVector = Vec::new();
        let mut newWeights: NBDistrict_WeightsCont = Vec::new();
        let mut joinedVal = 0.0;
        // go through the list of sources
        for (tmp, val) in self.mySources.iter().zip(self.mySourceWeights.iter()) {
            if !which.contains(tmp) {
                // if the current edge shall not be replaced, add to the temporary list
                newList.push(*tmp);
                newWeights.push(*val);
            } else {
                // otherwise, skip it and add its weight to the one to be inserted instead
                joinedVal += *val;
            }
        }
        // add the one to be inserted instead
        newList.push(by);
        newWeights.push(joinedVal);
        // assign to values
        self.mySources = newList;
        self.mySourceWeights = newWeights;
    }

    /// C++: `void NBDistrict::removeFromSinksAndSources(NBEdge *const)`
    /// Removes the given edge from the lists of sources and sinks
    // void
    // NBDistrict::removeFromSinksAndSources(NBEdge* const e) {
    //     int i;
    //     for (i = 0; i < (int)mySinks.size(); ++i) {
    //         if (mySinks[i] == e) {
    //             mySinks.erase(mySinks.begin() + i);
    //             mySinkWeights.erase(mySinkWeights.begin() + i);
    //         }
    //     }
    //     for (i = 0; i < (int)mySources.size(); ++i) {
    //         if (mySources[i] == e) {
    //             mySources.erase(mySources.begin() + i);
    //             mySourceWeights.erase(mySourceWeights.begin() + i);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_from_sinks_and_sources(&mut self, e: Ptr<NBEdge>) {
        // same index stepping as the C++ loop: after an erase the next element is skipped
        let mut i: usize = 0;
        while i < self.mySinks.len() {
            if self.mySinks[i] == e {
                self.mySinks.remove(i);
                self.mySinkWeights.remove(i);
            }
            i += 1;
        }
        i = 0;
        while i < self.mySources.len() {
            if self.mySources[i] == e {
                self.mySources.remove(i);
                self.mySourceWeights.remove(i);
            }
            i += 1;
        }
    }

    /// C++: `void NBDistrict::addShape(const PositionVector &)`
    /// Sets the shape of this district
    // void
    // NBDistrict::addShape(const PositionVector& p) {
    //     myShape = p;
    // }
    #[allow(unused_variables)]
    pub fn add_shape(&mut self, p: &PositionVector) {
        self.myShape = p.clone();
    }

    /// C++: `const std::vector<double> & NBDistrict::getSourceWeights() const` [const]
    /// Returns the weights of the sources
    // const std::vector<double>& getSourceWeights() const {
    //         return mySourceWeights;
    //     }
    #[allow(unused_variables)]
    pub fn get_source_weights(&self) -> &Vec<f64> {
        &self.mySourceWeights
    }

    /// C++: `const std::vector<NBEdge *> & NBDistrict::getSourceEdges() const` [const]
    /// Returns the sources
    // const std::vector<NBEdge*>& getSourceEdges() const {
    //         return mySources;
    //     }
    #[allow(unused_variables)]
    pub fn get_source_edges(&self) -> &Vec<Ptr<NBEdge>> {
        &self.mySources
    }

    /// C++: `const std::vector<double> & NBDistrict::getSinkWeights() const` [const]
    /// Returns the weights of the sinks
    // const std::vector<double>& getSinkWeights() const {
    //         return mySinkWeights;
    //     }
    #[allow(unused_variables)]
    pub fn get_sink_weights(&self) -> &Vec<f64> {
        &self.mySinkWeights
    }

    /// C++: `const std::vector<NBEdge *> & NBDistrict::getSinkEdges() const` [const]
    /// Returns the sinks
    // const std::vector<NBEdge*>& getSinkEdges() const {
    //         return mySinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_sink_edges(&self) -> &Vec<Ptr<NBEdge>> {
        &self.mySinks
    }

    /// C++: `const PositionVector & NBDistrict::getShape() const` [const]
    /// Returns the shape
    // const PositionVector& getShape() const {
    //         return myShape;
    //     }
    #[allow(unused_variables)]
    pub fn get_shape(&self) -> &PositionVector {
        &self.myShape
    }

    /// C++: `void NBDistrict::reshiftPosition(double, double)`
    /// Applies an offset to the district
    // void
    // NBDistrict::reshiftPosition(double xoff, double yoff) {
    //     myPosition.add(xoff, yoff, 0);
    //     myShape.add(xoff, yoff, 0);
    // }
    #[allow(unused_variables)]
    pub fn reshift_position(&mut self, xoff: f64, yoff: f64) {
        self.myPosition.add_xyz(xoff, yoff, 0.0);
        self.myShape.add_xyz(xoff, yoff, 0.0);
    }

    /// C++: `void NBDistrict::mirrorX()`
    /// mirror coordinates along the x-axis
    // void
    // NBDistrict::mirrorX() {
    //     myPosition.mul(1, -1);
    //     myShape.mirrorX();
    // }
    #[allow(unused_variables)]
    pub fn mirror_x(&mut self) {
        self.myPosition.mul_xy(1.0, -1.0);
        self.myShape.mirror_x();
    }

    /// C++: `void NBDistrict::NBDistrict(const NBDistrict &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &NBDistrict) -> Self {
        Self {
            base: Named::new(s.get_id()),
            mySources: s.mySources.clone(),
            mySourceWeights: s.mySourceWeights.clone(),
            mySinks: s.mySinks.clone(),
            mySinkWeights: s.mySinkWeights.clone(),
            myPosition: s.myPosition,
            myShape: s.myShape.clone(),
        }
    }

    /// C++: `NBDistrict & NBDistrict::operator=(const NBDistrict &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBDistrict) -> &mut NBDistrict {
        self.base.set_id(s.get_id());
        self.mySources = s.mySources.clone();
        self.mySourceWeights = s.mySourceWeights.clone();
        self.mySinks = s.mySinks.clone();
        self.mySinkWeights = s.mySinkWeights.clone();
        self.myPosition = s.myPosition;
        self.myShape = s.myShape.clone();
        self
    }

}


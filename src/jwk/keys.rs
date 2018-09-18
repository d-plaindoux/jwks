/**
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at
  http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

use jwk::signature::Capability;
use jwk::signature::Key;
use jwk::data::JSONWebKey;
use jwk::data::PublicSpecification;
use jwk::signature::Signature;
use jwk::data::PrivateSpecification;
use jwk::signature::Encryption;

/// KeyProvider is the main trait able to provide the key corresponding to a given specification

pub trait KeyProvider<E> where E: Capability {
    fn key(&self) -> Option<Key<E>>;
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Signature> for JSONWebKey<PublicSpecification> {
    fn key(&self) -> Option<Key<Signature>> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

impl KeyProvider<Encryption> for JSONWebKey<PrivateSpecification> {
    fn key(&self) -> Option<Key<Encryption>> {
        unimplemented!()
    }
}

// -------------------------------------------------------------------------------------------------

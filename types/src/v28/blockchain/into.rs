// SPDX-License-Identifier: CC0-1.0

use bitcoin::{Amount, BlockHash, ScriptBuf, Txid};

use super::{ScanTxOutSet, ScanTxOutSetStart, ScanTxOutSetStatus, ScanTxOutSetUnspent, ScanTxOutSetError};

use crate::model;

impl ScanTxOutSet {
    /// Converts version-specific `ScanTxOutSet` response into version-agnostic model type.
    pub fn into_model(self) -> Result<model::ScanTxOutSet, ScanTxOutSetError> {

        match self {
            Self::Start(start) => Ok(model::ScanTxOutSet::Start(start.into_model()?)),
            Self::Abort(b) => Ok(model::ScanTxOutSet::Abort(b)),
            Self::Status(s) => Ok(model::ScanTxOutSet::Status(s.map(|s| s.into_model()))),
        }
    }
}

impl ScanTxOutSetStart {
    pub fn into_model(self) -> Result<model::ScanTxOutSetStart, ScanTxOutSetError> {
        use ScanTxOutSetError as E;

        let bestblock = self
            .bestblock
            .parse::<BlockHash>().map_err(E::BestBlockHash)?;

        let unspents = self
            .unspents
            .into_iter()
            .map(|u| u.into_model())
            .collect::<Result<Vec<_>, _>>()?;

        let total_amount = Amount::from_btc(self.total_amount).map_err(E::TotalAmount)?;

        Ok(model::ScanTxOutSetStart {
            success: Some(self.success),
            txouts: Some(self.txouts),
            height: Some(self.height),
            bestblock: Some(bestblock),
            unspents,
            total_amount,
        })
    }
}

impl ScanTxOutSetUnspent {
    pub fn into_model(self) -> Result<model::ScanTxOutSetUnspent, ScanTxOutSetError> {
        use ScanTxOutSetError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let amount = Amount::from_btc(self.amount).map_err(E::Amount)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubKey)?;
        let blockhash = self
            .blockhash
            .parse::<BlockHash>().map_err(E::BlockHash)?;

        Ok(model::ScanTxOutSetUnspent {
            txid,
            vout: self.vout,
            script_pubkey,
            desc: Some(self.desc),
            amount,
            coinbase: Some(self.coinbase),
            height: self.height,
            blockhash: Some(blockhash),
            confirmations: Some(self.confirmations),
        })
    }
}

impl ScanTxOutSetStatus {
    pub fn into_model(self) -> model::ScanTxOutSetStatus {
        model::ScanTxOutSetStatus {
            progress: self.progress,
        }
    }
}

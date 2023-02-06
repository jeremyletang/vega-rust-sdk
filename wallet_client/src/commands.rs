use serde::{Deserialize, Serialize};

/// A batch of order instructions.
/// This command accepts only the following batches of commands
/// and will be processed in the following order:
/// - OrderCancellation
/// - OrderAmendment
/// - OrderSubmission
/// The total amount of commands in the batch across all three lists of
/// instructions is restricted by the following network parameter:
/// "spam.protection.max.batchSize"
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchMarketInstructions {
    /// A list of order cancellations to be processed sequentially
    pub cancellations: Vec<OrderCancellation>,
    /// A list of order amendments to be processed sequentially
    pub amendments: Vec<OrderAmendment>,
    /// A list of order submissions to be processed sequentially
    pub submissions: Vec<OrderSubmission>,
}

/// Time In Force for an order
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimeInForce {
    /// Default value for TimeInForce, can be valid for an amend
    #[serde(rename = "TIME_IN_FORCE_UNSPECIFIED")]
    Unspecified = 0,
    /// Good until cancelled, the order trades any amount and as much as possible
    /// and remains on the book until it either trades completely or is cancelled
    #[serde(rename = "TIME_IN_FORCE_GTC")]
    Gtc = 1,
    /// Good until specified time, this order type trades any amount and as much as possible
    /// and remains on the book until it either trades completely, is cancelled, or expires at a set time
    /// NOTE: this may in future be multiple types or have sub types for orders that provide different ways of specifying expiry
    #[serde(rename = "TIME_IN_FORCE_GTT")]
    Gtt = 2,
    /// Immediate or cancel, the order trades any amount and as much as possible
    /// but does not remain on the book (whether it trades or not)
    #[serde(rename = "TIME_IN_FORCE_IOC")]
    Ioc = 3,
    /// Fill or kill, The order either trades completely (remainingSize == 0 after adding)
    /// or not at all, does not remain on the book if it doesn't trade
    #[serde(rename = "TIME_IN_FORCE_FOK")]
    Fok = 4,
    /// Good for auction, this order is only accepted during an auction period
    #[serde(rename = "TIME_IN_FORCE_GFA")]
    Gfa = 5,
    /// Good for normal, this order is only accepted during normal trading (that can be continuous trading or frequent batched auctions)
    #[serde(rename = "TIME_IN_FORCE_GFN")]
    Gfn = 6,
}

/// Type values for an order
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    /// Default value, always invalid
    #[serde(rename = "TYPE_UNSPECIFIED")]
    Unspecified = 0,
    /// Used for Limit orders
    #[serde(rename = "TYPE_LIMIT")]
    Limit = 1,
    /// Used for Market orders
    #[serde(rename = "TYPE_MARKET")]
    Market = 2,
    /// Used for orders where the initiating party is the network (with distressed parties)
    #[serde(rename = "TYPE_NETWORK")]
    Network = 3,
}

/// A side relates to the direction of an order, to Buy, or Sell
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    /// Default value, always invalid
    #[serde(rename = "SIDE_UNSPECIFIED")]
    Unspecified = 0,
    /// Buy order
    #[serde(rename = "SIDE_BUY")]
    Buy = 1,
    /// Sell order
    #[serde(rename = "SIDE_SELL")]
    Sell = 2,
}

/// A pegged reference defines which price point a pegged order is linked to - meaning
/// the price for a pegged order is calculated from the value of the reference price point
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PeggedReference {
    /// Default value for PeggedReference, no reference given
    #[serde(rename = "PEGGED_REFERENCE_UNSPECIFIED")]
    Unspecified = 0,
    /// Mid price reference
    #[serde(rename = "PEGGED_REFERENCE_MID")]
    Mid = 1,
    /// Best bid price reference
    #[serde(rename = "PEGGED_REFERENCE_BEST_BID")]
    BestBid = 2,
    /// Best ask price reference
    #[serde(rename = "PEGGED_REFERENCE_BEST_ASK")]
    BestAsk = 3,
}

/// Pegged orders are limit orders where the price is specified in the form REFERENCE +/- OFFSET
/// They can be used for any limit order that is valid during continuous trading
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeggedOrder {
    /// The price point the order is linked to
    pub reference: PeggedReference,
    /// Offset from the price reference
    pub offset: String,
}

/// An order submission is a request to submit or create a new order on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderSubmission {
    /// Market identifier for the order, required field
    pub market_id: String,
    /// Price for the order, the price is an integer, for example `123456` is a correctly
    /// formatted price of `1.23456` assuming market configured to 5 decimal places,
    /// , required field for limit orders, however it is not required for market orders
    pub price: String,
    /// Size for the order, for example, in a futures market the size equals the number of units, cannot be negative
    pub size: u64,
    /// Side for the order, e.g. SIDE_BUY or SIDE_SELL, required field
    /// - See `Side`
    pub side: Side,
    /// Time in force indicates how long an order will remain active before it is executed or expires, required field
    /// - See `Order.TimeInForce`
    pub time_in_force: TimeInForce,
    /// Timestamp for when the order will expire, in nanoseconds since the epoch,
    /// required field only for `Order.TimeInForce`.TIME_IN_FORCE_GTT`
    /// - See `VegaTimeResponse`.`timestamp`
    pub expires_at: i64,
    /// Type for the order, required field - See `Order.Type`
    pub r#type: OrderType,
    /// Reference given for the order, this is typically used to retrieve an order submitted through consensus, currently
    /// set internally by the node to return a unique reference identifier for the order submission
    pub reference: String,
    /// Used to specify the details for a pegged order
    /// - See `PeggedOrder`
    pub pegged_order: Option<PeggedOrder>,
}
/// An order cancellation is a request to cancel an existing order on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCancellation {
    /// Unique identifier for the order (set by the system after consensus), required field
    pub order_id: String,
    /// Market identifier for the order, required field
    pub market_id: String,
}
/// An order amendment is a request to amend or update an existing order on Vega
///
/// The `orderID`, `partyID` and `marketID` fields are used for looking up the order only and cannot be amended by this command
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmendment {
    /// Order identifier, this is required to find the order and will not be updated, required field
    pub order_id: String,
    /// Market identifier, this is required to find the order and will not be updated
    pub market_id: String,
    /// Amend the price for the order, if the Price value is set, otherwise price will remain unchanged - See \[`Price`\](#vega.Price)
    pub price: Option<String>,
    /// Amend the size for the order by the delta specified:
    /// - To reduce the size from the current value set a negative integer value
    /// - To increase the size from the current value, set a positive integer value
    /// - To leave the size unchanged set a value of zero
    pub size_delta: i64,
    /// Amend the expiry time for the order, if the Timestamp value is set, otherwise expiry time will remain unchanged
    /// - See \[`VegaTimeResponse`\](#api.VegaTimeResponse).`timestamp`
    pub expires_at: Option<i64>,
    /// Amend the time in force for the order, set to TIME_IN_FORCE_UNSPECIFIED to remain unchanged
    /// - See \[`TimeInForce`\](#api.VegaTimeResponse).`timestamp`
    pub time_in_force: TimeInForce,
    /// Amend the pegged order offset for the order
    pub pegged_offset: String,
    /// Amend the pegged order reference for the order
    /// - See \[`PeggedReference`\](#vega.PeggedReference)
    pub pegged_reference: i32,
}

/// Represents a liquidity order
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityOrder {
    /// The pegged reference point for the order
    pub reference: PeggedReference,
    /// The relative proportion of the commitment to be allocated at a price level
    pub proportion: u32,
    /// The offset/amount of units away for the order
    pub offset: String,
}

/// A liquidity provision submitted for a given market
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityProvisionSubmission {
    /// Market identifier for the order, required field
    pub market_id: String,
    /// Specified as a unitless number that represents the amount of settlement asset of the market
    pub commitment_amount: String,
    /// Nominated liquidity fee factor, which is an input to the calculation of taker fees on the market, as per setting fees and rewarding liquidity providers
    pub fee: String,
    /// A set of liquidity sell orders to meet the liquidity provision obligation
    pub sells: Vec<LiquidityOrder>,
    /// A set of liquidity buy orders to meet the liquidity provision obligation
    pub buys: Vec<LiquidityOrder>,
    /// A reference to be added to every order created out of this liquidityProvisionSubmission
    pub reference: String,
}

/// Cancel a liquidity provision request
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityProvisionCancellation {
    /// Unique ID for the market with the liquidity provision to be cancelled
    pub market_id: String,
}

/// Amend a liquidity provision request
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityProvisionAmendment {
    /// Unique ID for the market with the liquidity provision to be amended
    pub market_id: String,
    /// an empty strings means no change
    pub commitment_amount: String,
    /// an empty strings means no change
    pub fee: String,
    /// empty slice means no change
    pub sells: Vec<LiquidityOrder>,
    /// empty slice means no change
    pub buys: Vec<LiquidityOrder>,
    /// empty string means no change
    pub reference: String,
}

/// An extension of data required for the withdraw submissions
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Erc20WithdrawExt {
    /// The address into which the bridge will release the funds
    pub receiver_address: String,
}

/// Foreign chain specifics
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WithdrawExt {
    /// ERC20 withdrawal details
    Erc20(Erc20WithdrawExt),
}

/// Represents the submission request to withdraw funds for a party on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawSubmission {
    /// The amount to be withdrawn
    pub amount: String,
    /// The asset to be withdrawn
    pub asset: String,
    /// Foreign chain specifics
    pub ext: Option<WithdrawExt>,
}

/// The rationale behind a proposal.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalRationale {
    /// Description to show a short title / something in case the link goes offline.
    /// This is to be between 0 and 20k unicode characters.
    /// This is mandatory for all proposals.
    pub description: String,
    /// Title to be used to give a short description of the proposal in lists.
    /// This is to be between 0 and 100 unicode characters.
    /// This is mandatory for all proposals.
    pub title: String,
}

/// DataSourceSpecConfigurationTime is the internal data source used for emitting timestamps.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceSpecConfigurationTime {
    /// Conditions that the timestamps should meet in order to be considered.
    pub conditions: Vec<DataCondition>,
}

/// DataSourceDefinitionInternal is the top level object used for all internal data sources.
/// It contains one of any of the defined `SourceType` variants.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceDefinitionInternal {
    /// Types of internal data sources
    pub source_type: Option<InternalSourceType>,
}

/// Types of internal data sources
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InternalSourceType {
    Time(DataSourceSpecConfigurationTime),
}

/// DataSourceDefinitionExternal is the top level object used for all external data sources.
/// It contains one of any of the defined `SourceType` variants.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceDefinitionExternal {
    /// Types of External data sources
    pub source_type: Option<ExternalSourceType>,
}

/// Types of External data sources
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExternalSourceType {
    Oracle(DataSourceSpecConfiguration),
}

/// Filter describes the conditions under which a data source data is considered of
/// interest or not.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataFilter {
    /// key is the data source data property key targeted by the filter.
    pub key: Option<DataPropertyKey>,
    /// conditions are the conditions that should be matched by the data to be
    /// considered of interest.
    pub conditions: Vec<DataCondition>,
}

/// PropertyKey describes the property key contained in an data source data.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPropertyKey {
    /// name is the name of the property.
    pub name: String,
    /// type is the type of the property.
    pub r#type: PropertyKeyType,
}

/// Condition describes the condition that must be validated by the network
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataCondition {
    /// comparator is the type of comparison to make on the value.
    pub operator: DataConditionOperator,
    /// value is used by the comparator.
    pub value: String,
}

/// Comparator describes the type of comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataConditionOperator {
    /// The default value
    #[serde(rename = "OPERATOR_UNSPECIFIED")]
    Unspecified = 0,
    /// Verify if the property values are strictly equal or not.
    #[serde(rename = "OPERATOR_EQUALS")]
    Equals = 1,
    /// Verify if the data source data value is greater than the Condition value.
    #[serde(rename = "OPERATOR_GREATER_THAN")]
    GreaterThan = 2,
    /// Verify if the data source data value is greater than or equal to the Condition
    /// value.
    #[serde(rename = "OPERATOR_GREATER_THAN_OR_EQUAL")]
    GreaterThanOrEqual = 3,
    /// Verify if the data source data value is less than the Condition value.
    #[serde(rename = "OPERATOR_LESS_THAN")]
    LessThan = 4,
    /// Verify if the data source data value is less or equal to than the Condition
    /// value.
    #[serde(rename = "OPERATOR_LESS_THAN_OR_EQUAL")]
    LessThanOrEqual = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Type describes the type of properties that are supported by the data source
/// engine.
pub enum PropertyKeyType {
    /// The default value.
    #[serde(rename = "TYPE_UNSPECIFIED")]
    Unspecified = 0,
    /// Any type.
    #[serde(rename = "TYPE_EMPTY")]
    Empty = 1,
    /// Integer type.
    #[serde(rename = "TYPE_INTEGER")]
    Integer = 2,
    /// String type.
    #[serde(rename = "TYPE_STRING")]
    String = 3,
    /// Boolean type.
    #[serde(rename = "TYPE_BOOLEAN")]
    Boolean = 4,
    /// Any floating point decimal type.
    #[serde(rename = "TYPE_DECIMAL")]
    Decimal = 5,
    /// Timestamp date type.
    #[serde(rename = "TYPE_TIMESTAMP")]
    Timestamp = 6,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthAddress {
    pub address: String,
}

/// PubKey is the public key that signed this data.
/// Different public keys coming from different sources will be further separated.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubKey {
    pub key: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSigner {
    pub signer: Option<DataSignerOneOff>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataSignerOneOff {
    /// pubKeys is the list of authorized public keys that signed the data for this
    /// source. All the public keys in the data should be contained in these
    /// public keys.
    PubKey(PubKey),
    /// in case of an open oracle - Ethereum address will be submitted
    EthAddress(EthAddress),
}

/// All types of external data sources use the same configuration set for meeting requirements
/// in order for the data to be useful for Vega - valid signatures and matching filters.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceSpecConfiguration {
    /// signers is the list of authorized signatures that signed the data for this
    /// source. All the signatures in the data source data should be contained in this
    /// external source. All the signatures in the data should be contained in this list.
    pub signers: Vec<DataSigner>,
    /// filters describes which source data are considered of interest or not for
    /// the product (or the risk model).
    pub filters: Vec<DataFilter>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SourceType {
    Internal(DataSourceDefinitionInternal),
    External(DataSourceDefinitionExternal),
}

/// DataSourceDefinition represents the top level object that deals with data sources.
/// DataSourceDefinition can be external or internal, with whatever number of data sources are defined
/// for each type in the child objects below.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceDefinition {
    pub source_type: Option<SourceType>,
}

/// Future product configuration
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureProduct {
    /// Asset ID for the product's settlement asset
    pub settlement_asset: String,
    /// Product quote name
    pub quote_name: String,
    /// The data source spec describing the data source for settlement
    pub data_source_spec_for_settlement_data: Option<DataSourceDefinition>,
    /// The external data source spec describing the data source of trading termination
    pub data_source_spec_for_trading_termination: Option<DataSourceDefinition>,
    /// The binding between the data source spec and the settlement data
    pub data_source_spec_binding: Option<DataSourceSpecToFutureBinding>,
    /// The number of decimal places implied by the settlement data (such as price) emitted by the settlement data source
    pub settlement_data_decimals: u32,
}

/// Product specification
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Product {
    /// Future
    Future(FutureProduct),
}

/// Instrument configuration
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentConfiguration {
    /// Instrument name
    pub name: String,
    /// Instrument code, human-readable shortcode used to describe the instrument
    pub code: String,
    /// Product specification
    pub product: Option<Product>,
}

/// PriceMonitoringTrigger holds together price projection horizon τ, probability level p, and auction extension duration
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceMonitoringTrigger {
    /// Price monitoring projection horizon τ in seconds
    pub horizon: i64,
    /// Price monitoring probability level p
    pub probability: String,
    /// Price monitoring auction extension duration in seconds should the price
    /// breach its theoretical level over the specified horizon at the specified
    /// probability level
    pub auction_extension: i64,
}

/// PriceMonitoringParameters contains a collection of triggers to be used for a given market
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceMonitoringParameters {
    pub triggers: Vec<PriceMonitoringTrigger>,
}

/// LiquidityMonitoringParameters contains settings used for liquidity monitoring
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityMonitoringParameters {
    /// Specifies parameters related to target stake calculation
    pub target_stake_parameters: Option<TargetStakeParameters>,
    /// Specifies the triggering ratio for entering liquidity auction
    pub triggering_ratio: f64,
    /// Specifies by how many seconds an auction should be extended if leaving the auction were to trigger a liquidity auction
    pub auction_extension: i64,
}
/// TargetStakeParameters contains parameters used in target stake calculation
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetStakeParameters {
    /// Specifies length of time window expressed in seconds for target stake calculation
    pub time_window: i64,
    /// Specifies scaling factors used in target stake calculation
    pub scaling_factor: f64,
}

/// Configuration for a new market on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMarketConfiguration {
    /// New market instrument configuration
    pub instrument: Option<InstrumentConfiguration>,
    /// Decimal places used for the new market, sets the smallest price increment on the book
    pub decimal_places: u64,
    /// Optional new market meta data, tags
    pub metadata: Vec<String>,
    /// Price monitoring parameters
    pub price_monitoring_parameters: Option<PriceMonitoringParameters>,
    /// Liquidity monitoring parameters
    pub liquidity_monitoring_parameters: Option<LiquidityMonitoringParameters>,
    /// Decimal places for order sizes, sets what size the smallest order / position on the market can be
    pub position_decimal_places: i64,
    /// Percentage move up and down from the mid price which specifies the range of
    /// price levels over which automated liquidity provision orders will be deployed
    pub lp_price_range: String,
    /// New market risk model parameters
    pub risk_parameters: Option<RiskParameters>,
}

/// New market on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMarket {
    /// The configuration of the new market
    pub changes: Option<NewMarketConfiguration>,
}

/// Update an existing market on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMarket {
    /// The identifier of the market to update
    pub market_id: String,
    /// The updated configuration of the market
    pub changes: Option<UpdateMarketConfiguration>,
}

/// Configuration to update a market on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMarketConfiguration {
    /// Updated market instrument configuration
    pub instrument: Option<UpdateInstrumentConfiguration>,
    /// Optional market metadata, tags
    pub metadata: Vec<String>,
    /// Price monitoring parameters
    pub price_monitoring_parameters: Option<PriceMonitoringParameters>,
    /// Liquidity monitoring parameters
    pub liquidity_monitoring_parameters: Option<LiquidityMonitoringParameters>,
    /// Percentage move up and down from the mid price which specifies the range of
    /// price levels over which automated liquidity provision orders will be deployed
    pub lp_price_range: String,
    /// Updated market risk model parameters
    pub risk_parameters: Option<RiskParameters>,
}

/// Risk model parameters for log normal
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogNormalModelParams {
    /// Mu parameter, annualised growth rate of the underlying asset
    pub mu: f64,
    /// R parameter, annualised growth rate of the risk-free asset, used for discounting of future cash flows, can be any real number
    pub r: f64,
    /// Sigma parameter, annualised volatility of the underlying asset, must be a strictly non-negative real number
    pub sigma: f64,
}

/// Risk model for log normal
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogNormalRiskModel {
    /// Risk Aversion Parameter
    pub risk_aversion_parameter: f64,
    /// Tau parameter of the risk model, projection horizon measured as a year fraction used in the expected shortfall calculation to obtain the maintenance margin, must be a strictly non-negative real number
    pub tau: f64,
    /// Risk model parameters for log normal
    pub params: Option<LogNormalModelParams>,
}

/// Updated market risk model parameters
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RiskParameters {
    /// Log normal risk model parameters, valid only if MODEL_LOG_NORMAL is selected
    LogNormal(LogNormalRiskModel),
}

/// Product specification
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateProduct {
    /// Future
    Future(UpdateFutureProduct),
}

/// Instrument configuration
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInstrumentConfiguration {
    /// Instrument code, human-readable shortcode used to describe the instrument
    pub code: String,
    /// Product specification
    pub product: Option<UpdateProduct>,
}

/// DataSourceSpecToFutureBinding describes which property of the data source data is to be
/// used as settlement data and which to use as the trading terminated trigger
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceSpecToFutureBinding {
    /// settlement_data_property holds the name of the property in the source data
    /// that should be used as settlement data.
    /// If it is set to "prices.BTC.value", then the Future will use the value of
    /// this property as settlement data.
    pub settlement_data_property: String,
    /// the name of the property in the data source data that signals termination of trading
    pub trading_termination_property: String,
}

/// Future product configuration
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFutureProduct {
    /// Human-readable name/abbreviation of the quote name
    pub quote_name: String,
    /// The data source spec describing the data of settlement data
    pub data_source_spec_for_settlement_data: Option<DataSourceDefinition>,
    /// The data source spec describing the data source for trading termination
    pub data_source_spec_for_trading_termination: Option<DataSourceDefinition>,
    /// The binding between the data source spec and the settlement data
    pub data_source_spec_binding: Option<DataSourceSpecToFutureBinding>,
    /// The number of decimal places implied by the settlement data (such as price) emitted by the settlement external data source
    pub settlement_data_decimals: u32,
}

/// Represents a network parameter on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkParameter {
    /// The unique key
    pub key: String,
    /// The value for the network parameter
    pub value: String,
}

/// Update network configuration on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNetworkParameter {
    /// The network parameter to update
    pub changes: Option<NetworkParameter>,
}

/// An ERC20 token based asset, living on the ethereum network
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Erc20Asset {
    /// The address of the contract for the token, on the ethereum network
    pub contract_address: String,
    /// The lifetime limits deposit per address
    /// note: this is a temporary measure that can be changed by governance
    pub lifetime_limit: String,
    /// The maximum you can withdraw instantly. All withdrawals over the threshold will be delayed by the withdrawal delay.
    /// There’s no limit on the size of a withdrawal
    /// note: this is a temporary measure that can be changed by governance
    pub withdraw_threshold: String,
}

/// The source
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssetSource {
    /// An Ethereum ERC20 asset
    Erc20(Erc20Asset),
}

/// The Vega representation of an external asset
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetails {
    /// Name of the asset (e.g: Great British Pound)
    pub name: String,
    /// Symbol of the asset (e.g: GBP)
    pub symbol: String,
    /// Number of decimal / precision handled by this asset
    pub decimals: u64,
    /// The minimum economically meaningful amount in the asset
    pub quantum: String,
    /// The source
    pub source: Option<AssetSource>,
}

/// New asset on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAsset {
    /// The configuration of the new asset
    pub changes: Option<AssetDetails>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Erc20AssetUpdate {
    /// The lifetime limits deposit per address.
    /// This is will be interpreted against the asset decimals.
    /// note: this is a temporary measure that can be changed by governance
    pub lifetime_limit: String,
    /// The maximum you can withdraw instantly. All withdrawals over the threshold will be delayed by the withdrawal delay.
    /// There’s no limit on the size of a withdrawal
    /// note: this is a temporary measure that can be changed by governance
    pub withdraw_threshold: String,
}

/// The source
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssetUpdateSource {
    /// An Ethereum ERC20 asset
    Erc20(Erc20AssetUpdate),
}

/// The changes to apply on an existing asset.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetailsUpdate {
    /// The minimum economically meaningful amount in the asset
    pub quantum: String,
    /// The source
    pub source: Option<AssetUpdateSource>,
}

/// Update an existing asset on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAsset {
    /// The ID of the asset to be updated
    pub asset_id: String,
    /// The changes to apply on an existing asset
    pub changes: Option<AssetDetailsUpdate>,
}

/// Freeform proposal
/// This message is just used as a placeholder to sort out the nature of the
/// proposal once parsed.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFreeform {}

/// Changes being proposed via governance
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProposalChange {
    /// Proposal change for modifying an existing market on Vega
    UpdateMarket(UpdateMarket),
    /// Proposal change for creating new market on Vega
    NewMarket(NewMarket),
    /// Proposal change for updating Vega network parameters
    UpdateNetworkParameter(UpdateNetworkParameter),
    /// Proposal change for creating new assets on Vega
    NewAsset(NewAsset),
    /// Proposal change for a freeform request, which can be voted on but does not change the behaviour of the system,
    /// and can be used to gauge community sentiment
    NewFreeform(NewFreeform),
    /// Proposal change for updating an asset
    UpdateAsset(UpdateAsset),
}

/// Terms for a governance proposal on Vega
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalTerms {
    /// Timestamp (Unix time in seconds) when voting closes for this proposal,
    /// constrained by `minClose` and `maxClose` network parameters
    pub closing_timestamp: i64,
    /// Timestamp (Unix time in seconds) when proposal gets enacted (if passed),
    /// constrained by `minEnact` and `maxEnact` network parameters
    pub enactment_timestamp: i64,
    /// Validation timestamp (Unix time in seconds)
    pub validation_timestamp: i64,
    /// Changes being proposed
    pub change: Option<ProposalChange>,
}

/// A command to submit a new proposal for the
/// Vega network governance
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalSubmission {
    /// Proposal reference
    pub reference: String,
    /// Proposal configuration and the actual change that is meant to be executed when proposal is enacted
    pub terms: Option<ProposalTerms>,
    /// The rationale behind a proposal.
    pub rationale: Option<ProposalRationale>,
}

/// Vote value
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum VoteValue {
    /// Default value, always invalid
    #[serde(rename = "VALUE_UNSPECIFIED")]
    Unspecified = 0,
    /// A vote against the proposal
    #[serde(rename = "VALUE_NO")]
    No = 1,
    /// A vote in favour of the proposal
    #[serde(rename = "VALUE_YES")]
    Yes = 2,
}

/// A command to submit a new vote for a governance
/// proposal.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteSubmission {
    /// The ID of the proposal to vote for.
    pub proposal_id: String,
    /// The actual value of the vote
    pub value: VoteValue,
}

/// A command to submit an instruction to delegate some stake to a node
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateSubmission {
    /// The ID for the node to delegate to
    pub node_id: String,
    /// The amount of stake to delegate
    pub amount: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndelegateSubmission {
    pub node_id: String,
    /// optional, if not specified = ALL
    pub amount: String,
    pub method: UndelegateMethod,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UndelegateMethod {
    #[serde(rename = "METHOD_UNSPECIFIED")]
    Unspecified = 0,
    #[serde(rename = "METHOD_NOW")]
    Now = 1,
    #[serde(rename = "METHOD_AT_END_OF_EPOCH")]
    AtEndOfEpoch = 2,
}

/// Various collateral/account types as used by Vega
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    /// Default value
    #[serde(rename = "ACCOUNT_TYPE_UNSPECIFIED")]
    Unspecified = 0,
    /// Insurance pool accounts contain insurance pool funds for a market
    #[serde(rename = "ACCOUNT_TYPE_INSURANCE")]
    Insurance = 1,
    /// Settlement accounts exist only during settlement or mark-to-market
    #[serde(rename = "ACCOUNT_TYPE_SETTLEMENT")]
    Settlement = 2,
    /// Margin accounts contain funds set aside for the margin needed to support a party's open positions.
    /// Each party will have a margin account for each market they have traded in.
    /// The required initial margin is allocated to each market from your general account.
    /// Collateral in the margin account can't be withdrawn or used as margin on another market until
    /// it is released back to the general account.
    /// The Vega protocol uses an internal accounting system to segregate funds held as
    /// margin from other funds to ensure they are never lost or 'double spent'
    ///
    /// Margin account funds will vary as margin requirements on positions change
    #[serde(rename = "ACCOUNT_TYPE_MARGIN")]
    Margin = 3,
    /// General accounts contain the collateral for a party that is not otherwise allocated. A party will
    /// have multiple general accounts, one for each asset they want
    /// to trade with
    ///
    /// General accounts are where funds are initially deposited or withdrawn from,
    /// it is also the account where funds are taken to fulfil fees and initial margin requirements
    #[serde(rename = "ACCOUNT_TYPE_GENERAL")]
    General = 4,
    /// Infrastructure accounts contain fees earned by providing infrastructure on Vega
    #[serde(rename = "ACCOUNT_TYPE_FEES_INFRASTRUCTURE")]
    FeesInfrastructure = 5,
    /// Liquidity accounts contain fees earned by providing liquidity on Vega markets
    #[serde(rename = "ACCOUNT_TYPE_FEES_LIQUIDITY")]
    FeesLiquidity = 6,
    /// This account is created to hold fees earned by placing orders that sit on the book
    /// and are then matched with an incoming order to create a trade - These fees reward parties
    /// who provide the best priced liquidity that actually allows trading to take place
    #[serde(rename = "ACCOUNT_TYPE_FEES_MAKER")]
    FeesMaker = 7,
    /// This account is created to maintain liquidity providers funds commitments
    #[serde(rename = "ACCOUNT_TYPE_BOND")]
    Bond = 9,
    /// External account represents an external source (deposit/withdrawal)
    #[serde(rename = "ACCOUNT_TYPE_EXTERNAL")]
    External = 10,
    /// Global insurance account for the asset
    #[serde(rename = "ACCOUNT_TYPE_GLOBAL_INSURANCE")]
    GlobalInsurance = 11,
    /// Global reward account for the asset
    #[serde(rename = "ACCOUNT_TYPE_GLOBAL_REWARD")]
    GlobalReward = 12,
    /// Per asset account used to store pending transfers (if any)
    #[serde(rename = "ACCOUNT_TYPE_PENDING_TRANSFERS")]
    PendingTransfers = 13,
    /// Per asset reward account for fees paid to makers
    #[serde(rename = "ACCOUNT_TYPE_REWARD_MAKER_PAID_FEES")]
    RewardMakerPaidFees = 14,
    /// Per asset reward account for fees received by makers
    #[serde(rename = "ACCOUNT_TYPE_REWARD_MAKER_RECEIVED_FEES")]
    RewardMakerReceivedFees = 15,
    /// Per asset reward account for fees received by liquidity providers
    #[serde(rename = "ACCOUNT_TYPE_REWARD_LP_RECEIVED_FEES")]
    RewardLpReceivedFees = 16,
    /// Per asset reward account for market proposers when the market goes above some trading threshold
    #[serde(rename = "ACCOUNT_TYPE_REWARD_MARKET_PROPOSERS")]
    RewardMarketProposers = 17,
}

/// A transfer initiated by a party
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    /// The account type from which the funds of the party
    /// should be taken
    pub from_account_type: AccountType,
    /// The public key of the destination account
    pub to: String,
    /// The type of the destination account
    pub to_account_type: AccountType,
    /// The asset
    pub asset: String,
    /// The amount to be taken from the source account
    pub amount: String,
    /// The reference to be attached to the transfer
    pub reference: String,
    /// Specific details of the transfer
    pub kind: Option<TransferKind>,
}

/// Specific details for a one off transfer
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneOffTransfer {
    /// A unix timestamp in second. Time at which the
    /// transfer should be delivered in the to account
    pub deliver_on: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DispatchMetric {
    #[serde(rename = "DISPATCH_METRIC_UNSPECIFIED")]
    Unspecified = 0,
    /// Dispatch metric that is using the total maker fees paid in the market
    #[serde(rename = "DISPATCH_METRIC_MAKER_FEES_PAID")]
    MakerFeesPaid = 1,
    /// Dispatch metric that is using the total maker fees received in the market
    #[serde(rename = "DISPATCH_METRIC_MAKER_FEES_RECEIVED")]
    MakerFeesReceived = 2,
    /// Dispatch metric that is using the total LP fees received in the market
    #[serde(rename = "DISPATCH_LP_FEES_RECEIVED")]
    LpFeesReceived = 3,
    /// Dispatch metric that is using total value of the market if above the required threshold and not paid given proposer bonus yet
    #[serde(rename = "DISPATCH_METRIC_MARKET_VALUE")]
    MarketValue = 4,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DispatchStrategy {
    /// The asset to use for metric
    pub asset_for_metric: String,
    /// The metric to apply
    pub metric: DispatchMetric,
    /// Optional markets in scope
    pub markets: Vec<String>,
}

/// Specific details for a recurring transfer
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringTransfer {
    /// The first epoch from which this transfer shall be paid
    pub start_epoch: u64,
    /// The last epoch at which this transfer shall be paid
    pub end_epoch: ::core::option::Option<u64>,
    /// factor needs to be > 0
    pub factor: String,
    /// optional parameter defining how a transfer is dispatched
    pub dispatch_strategy: Option<DispatchStrategy>,
}

/// Specific details of the transfer
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransferKind {
    OneOff(OneOffTransfer),
    Recurring(RecurringTransfer),
}

/// A request for cancelling a recurring transfer
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTransfer {
    /// The ID of the transfer to cancel
    pub transfer_id: String,
}

/// Command to submit new Oracle data from third party providers
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleDataSubmission {
    /// The source from which the data is coming from. Must be base64 encoded.
    /// Oracle data a type of external data source data.
    pub source: OracleSource,
    /// The data provided by the data source
    /// In the case of Open Oracle - it will be the entire object - it will contain messages, signatures and price data
    pub payload: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OracleSource {
    /// The default value
    #[serde(rename = "ORACLE_SOURCE_UNSPECIFIED")]
    Unspecified = 0,
    /// Specifies that the payload will be base64 encoded JSON conforming to the Open Oracle standard
    #[serde(rename = "ORACLE_SOURCE_OPEN_ORACLE")]
    OpenOracle = 1,
    /// Specifies that the payload will be base64 encoded JSON, but does not specify the shape of the data
    #[serde(rename = "ORACLE_SOURCE_JSON")]
    Json = 2,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Command {
    BatchMarketInstructions(BatchMarketInstructions),
    OrderSubmission(OrderSubmission),
    OrderCancellation(OrderCancellation),
    OrderAmendment(OrderAmendment),
    LiquidityProvisionSubmission(LiquidityProvisionSubmission),
    LiquidityProvisionCancellation(LiquidityProvisionCancellation),
    LiquidityProvisionAmendment(LiquidityProvisionAmendment),
    WithdrawSubmission(WithdrawSubmission),
    ProposalSubmission(ProposalSubmission),
    VoteSubmission(VoteSubmission),
    DelegateSubmisson(DelegateSubmission),
    UndelegateSubmission(UndelegateSubmission),
    Transfer(Transfer),
    CancelTransfer(CancelTransfer),
    OracleDataSubmission(OracleDataSubmission),
}

impl From<BatchMarketInstructions> for Command {
    fn from(cmd: BatchMarketInstructions) -> Self {
        Command::BatchMarketInstructions(cmd)
    }
}

impl From<OrderSubmission> for Command {
    fn from(cmd: OrderSubmission) -> Self {
        Command::OrderSubmission(cmd)
    }
}

impl From<OrderCancellation> for Command {
    fn from(cmd: OrderCancellation) -> Self {
        Command::OrderCancellation(cmd)
    }
}

impl From<OrderAmendment> for Command {
    fn from(cmd: OrderAmendment) -> Self {
        Command::OrderAmendment(cmd)
    }
}

impl From<LiquidityProvisionSubmission> for Command {
    fn from(cmd: LiquidityProvisionSubmission) -> Self {
        Command::LiquidityProvisionSubmission(cmd)
    }
}

impl From<LiquidityProvisionCancellation> for Command {
    fn from(cmd: LiquidityProvisionCancellation) -> Self {
        Command::LiquidityProvisionCancellation(cmd)
    }
}

impl From<LiquidityProvisionAmendment> for Command {
    fn from(cmd: LiquidityProvisionAmendment) -> Self {
        Command::LiquidityProvisionAmendment(cmd)
    }
}

impl From<WithdrawSubmission> for Command {
    fn from(cmd: WithdrawSubmission) -> Self {
        Command::WithdrawSubmission(cmd)
    }
}

impl From<ProposalSubmission> for Command {
    fn from(cmd: ProposalSubmission) -> Self {
        Command::ProposalSubmission(cmd)
    }
}

impl From<VoteSubmission> for Command {
    fn from(cmd: VoteSubmission) -> Self {
        Command::VoteSubmission(cmd)
    }
}

impl From<DelegateSubmission> for Command {
    fn from(cmd: DelegateSubmission) -> Self {
        Command::DelegateSubmisson(cmd)
    }
}

impl From<UndelegateSubmission> for Command {
    fn from(cmd: UndelegateSubmission) -> Self {
        Command::UndelegateSubmission(cmd)
    }
}

impl From<OracleDataSubmission> for Command {
    fn from(cmd: OracleDataSubmission) -> Self {
        Command::OracleDataSubmission(cmd)
    }
}

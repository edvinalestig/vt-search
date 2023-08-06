# JourneyDetailsPeriodTicketSuggestionApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | Option<**i32**> | The product id. | [optional]
**product_name** | Option<**String**> | The product name. | [optional]
**product_type** | Option<**i32**> | The product type. | [optional]
**traveller_category** | Option<[**crate::models::TravellerCategory**](VT.ApiPlaneraResa.Core.Models.TravellerCategory.md)> |  | [optional]
**price_in_sek** | Option<**f64**> | The product price in SEK. | [optional]
**time_validity** | Option<[**crate::models::JourneyDetailsPeriodTimeValidityApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.TimeValidityApiModel.md)> |  | [optional]
**time_limitation** | Option<[**crate::models::TimeLimitation**](VT.ApiPlaneraResa.Core.Models.TimeLimitation.md)> |  | [optional]
**sale_channels** | Option<[**Vec<crate::models::JourneyDetailsPeriodChannelApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.ChannelApiModel.md)> | A list of the channels that sell the product. | [optional]
**valid_zones** | Option<[**Vec<crate::models::JourneyDetailsPeriodZoneApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.ZoneApiModel.md)> | A list of the valid zones for the ticket. | [optional]
**product_instance_type** | Option<[**crate::models::ProductInstanceTypeApiModel**](VT.ApiPlaneraResa.Web.V4.Models.ProductInstanceTypeApiModel.md)> |  | [optional]
**punch_configuration** | Option<[**crate::models::JourneyDetailsPeriodPunchConfigurationApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.PunchConfigurationApiModel.md)> |  | [optional]
**offer_specification** | Option<**String**> | Used to get ticket offer. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



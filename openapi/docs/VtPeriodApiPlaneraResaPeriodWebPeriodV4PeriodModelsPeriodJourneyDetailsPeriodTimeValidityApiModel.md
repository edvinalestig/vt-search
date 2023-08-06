# JourneyDetailsPeriodTimeValidityApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<[**crate::models::TimeValidityType**](VT.ApiPlaneraResa.Core.Models.TimeValidityType.md)> |  | [optional]
**amount** | Option<**i32**> | The amount of the unit specified by the Unit property. Always used together with the Unit property. | [optional]
**unit** | Option<[**crate::models::TimeValidityUnit**](VT.ApiPlaneraResa.Core.Models.TimeValidityUnit.md)> |  | [optional]
**from_date** | Option<**String**> | The from date of a date interval specified in RFC 3339 format. Always used together with the  ToDate property. | [optional]
**to_date** | Option<**String**> | The to date of a date interval specified in RFC 3339 format. Always used together with the  FromDate property. | [optional]
**from_date_time** | Option<**String**> | The from time of a datetime interval specified in RFC 3339 format. Always used together with  the ToDateTime property. | [optional]
**to_date_time** | Option<**String**> | The to time of a datetime interval specified in RFC 3339 format. Always used together with  the FromDateTime property. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



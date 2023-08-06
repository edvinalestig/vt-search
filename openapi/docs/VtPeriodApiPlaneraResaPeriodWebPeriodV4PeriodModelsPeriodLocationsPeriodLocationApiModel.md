# LocationsPeriodLocationApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gid** | Option<**String**> | The 16-digit Västtrafik gid. | [optional]
**name** | **String** | The location name. | 
**location_type** | [**crate::models::LocationType**](VT.ApiPlaneraResa.Core.Models.LocationType.md) |  | 
**latitude** | Option<**f64**> | The WGS84 latitude of the location. | [optional]
**longitude** | Option<**f64**> | The WGS84 longitude of the location. | [optional]
**platform** | Option<**String**> | The location platform, only available for stop points. | [optional]
**straight_line_distance_in_meters** | Option<**i32**> | The location straight line distance in meters. | [optional]
**has_local_service** | Option<**bool**> | Is \"Närtrafik\" (Local Service) available for the location?  Values are only available for LocationType: StopArea, PointOfInterest and Address.  Values are only available for endpoint: locations/by-text. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



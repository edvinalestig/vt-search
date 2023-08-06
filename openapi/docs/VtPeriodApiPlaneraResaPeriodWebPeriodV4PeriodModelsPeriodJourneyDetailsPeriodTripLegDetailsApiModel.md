# JourneyDetailsPeriodTripLegDetailsApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_journeys** | Option<[**Vec<crate::models::JourneyDetailsPeriodServiceJourneyApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.ServiceJourneyApiModel.md)> | The service journey for the trip leg. | [optional]
**calls_on_trip_leg** | Option<[**Vec<crate::models::JourneyDetailsPeriodCallDetailsApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.CallDetailsApiModel.md)> | The calls on the trip leg. | [optional]
**trip_leg_coordinates** | Option<[**Vec<crate::models::CoordinateApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.CoordinateApiModel.md)> | The coordinates for the trip leg. | [optional]
**tariff_zones** | Option<[**Vec<crate::models::JourneyDetailsPeriodTariffZoneApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.TariffZoneApiModel.md)> | The tariff zones that the trip leg traverses. | [optional]
**is_cancelled** | Option<**bool**> | Flag indicating if the trip leg is cancelled. | [optional]
**is_part_cancelled** | Option<**bool**> | Flag indicating if the trip leg is partially cancelled. | [optional]
**occupancy** | Option<[**crate::models::OccupancyInformationApiModel**](VT.ApiPlaneraResa.Web.V4.Models.OccupancyInformationApiModel.md)> |  | [optional]
**journey_leg_index** | Option<**i32**> | Index of Leg in Journey | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



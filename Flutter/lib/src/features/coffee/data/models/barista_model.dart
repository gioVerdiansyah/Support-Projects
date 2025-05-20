
import 'package:coffe_shop/src/features/barista/domain/entities/barista_entity.dart';

class BaristaModel extends BaristaEntity {


  BaristaModel();

  factory BaristaModel.fromJson(Map<String, dynamic> json) => BaristaModel(

  );

  BaristaEntity toEntity() {
    return BaristaEntity();
  }
}